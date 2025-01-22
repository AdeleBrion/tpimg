use image::{io::Reader as ImageReader, ImageError, Luma, Pixel, Rgb, RgbImage};
use rand::Rng; // Pour générer des valeurs aléatoires

// Fonctions pour le point 2

/// Calcule la luminosité d'un pixel et le convertit en une couleur personnalisée
/// en fonction de sa luminosité, en utilisant deux couleurs (clair et foncé).
fn pixel_luminosity_to_custom(color: &Rgb<u8>, light_color: Rgb<u8>, dark_color: Rgb<u8>) -> Rgb<u8> {
    let luminosite = 0.2126 * color[0] as f32 + 0.7152 * color[1] as f32 + 0.0722 * color[2] as f32;
    if luminosite > 127.5 {
        light_color
    } else {
        dark_color
    }
}

/// Récupère la luminosité d'un pixel spécifique d'une image RGB.
fn get_pixel_luminosity(img: &RgbImage, x:u32, y:u32) -> u8 {
    let Luma(luminosite_) = img.get_pixel(x, y).to_luma();
    return luminosite_[0];
}


// Fonction pour le point 3

/// Fonction pour calculer la distance Euclidienne entre deux couleurs
fn color_distance(c1: &Rgb<u8>, c2: &Rgb<u8>) -> f32 {
    let r_diff = (c1[0] as f32 - c2[0] as f32).powi(2);
    let g_diff = (c1[1] as f32 - c2[1] as f32).powi(2);
    let b_diff = (c1[2] as f32 - c2[2] as f32).powi(2);
    (r_diff + g_diff + b_diff).sqrt()
}

/// Fonction pour trouver la couleur la plus proche dans la palette
fn pixel_to_palette(pixel: &Rgb<u8>, palette: &[Rgb<u8>]) -> Rgb<u8> {
    let mut closest_color = &palette[0];
    let mut min_distance = color_distance(pixel, &palette[0]);
    
    for color in palette.iter() {
        let dist = color_distance(pixel, color);
        if dist < min_distance {
            closest_color = color;
            min_distance = dist;
        }
    }

    return *closest_color
}


// Fonction pour le point 4

/// Applique un tramage aléatoire (dithering) à une image RGB, en convertissant chaque pixel
/// en noir ou blanc en fonction d'un seuil aléatoire.
fn apply_random_dithering(img: &mut RgbImage) {
    let mut rng = rand::thread_rng();
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let threshold: f32 = rng.gen_range(0.0..1.0); 
        let luminosity = 0.2126 * pixel[0] as f32 + 0.7152 * pixel[1] as f32 + 0.0722 * pixel[2] as f32;
        
        if luminosity > threshold * 255.0 {
            *pixel = Rgb([255, 255, 255]);
        } else {
            *pixel = Rgb([0, 0, 0]);
        }
    }
}

// Fonction pour le point 5
fn generate_bayer_matrix(order: u8) -> Vec<Vec<u8>> {
    if order == 0 {
        return vec![vec![0]];
    }
    let prev_matrix = generate_bayer_matrix(order - 1);
    let size = prev_matrix.len();
    let mut matrix = vec![vec![0; size * 2]; size * 2];

    for i in 0..size {
        for j in 0..size {
            matrix[i][j] = 4 * prev_matrix[i][j];
            matrix[i][j + size] = 4 * prev_matrix[i][j] + 2;
            matrix[i + size][j] = 4 * prev_matrix[i][j] + 3;
            matrix[i + size][j + size] = 4 * prev_matrix[i][j] + 1;
        }
    }
    matrix
}

/// Fonction pour appliquer le tramage par matrice de Bayer
fn apply_bayer_dithering(img: &RgbImage, order: u8) -> RgbImage {
    let bayer_matrix = generate_bayer_matrix(order);
    let size = bayer_matrix.len();
    let mut output = img.clone();

    for (x, y, pixel) in img.enumerate_pixels() {
        let threshold = bayer_matrix[(y as usize % size)][(x as usize % size)];
        let normalized_threshold = (threshold as f32 / (size * size - 1) as f32) * 255.0;
        let luminosity = 0.2126 * pixel[0] as f32 + 0.7152 * pixel[1] as f32 + 0.0722 * pixel[2] as f32;

        if luminosity > normalized_threshold {
            output.put_pixel(x, y, Rgb([255, 255, 255]));
        } else {
            output.put_pixel(x, y, Rgb([0, 0, 0]));
        }
    }

    output
}

// Fonction pour le point 6
fn apply_error_diffusion_dithering(img: &mut RgbImage) {
    let width = img.width();
    let height = img.height();
    
    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let luminosity = 0.2126 * pixel[0] as f32 + 0.7152 * pixel[1] as f32 + 0.0722 * pixel[2] as f32;
            
            let new_color = if luminosity > 127.5 { Rgb([255, 255, 255]) } else { Rgb([0, 0, 0]) };
            let error = luminosity - (if luminosity > 127.5 { 255.0 } else { 0.0 });
            
            if x + 1 < width {
                let mut neighbor = img.get_pixel_mut(x + 1, y);
                let neighbor_error = (error * 0.5) as u8;
                *neighbor = Rgb([neighbor[0].saturating_add(neighbor_error), neighbor[1].saturating_add(neighbor_error), neighbor[2].saturating_add(neighbor_error)]);
            }
            if y + 1 < height {
                let mut neighbor = img.get_pixel_mut(x, y + 1);
                let neighbor_error = (error * 0.5) as u8;
                *neighbor = Rgb([neighbor[0].saturating_add(neighbor_error), neighbor[1].saturating_add(neighbor_error), neighbor[2].saturating_add(neighbor_error)]);
            }
            img.put_pixel(x, y, new_color);
        }
    }
}

fn closest_palette_color(pixel: &Rgb<u8>) -> Rgb<u8> {
    let palette = [
        Rgb([0, 0, 0]),       // Noir
        Rgb([255, 255, 255]), // Blanc
        Rgb([255, 0, 0]),     // Rouge
        Rgb([0, 0, 255]),     // Bleu
        Rgb([0, 255, 0]),     // Vert
    ];

    let mut min_dist = f32::MAX;
    let mut closest_color = Rgb([0, 0, 0]);

    for &color in &palette {
        let dist = ((pixel[0] as f32 - color[0] as f32).powi(2)
            + (pixel[1] as f32 - color[1] as f32).powi(2)
            + (pixel[2] as f32 - color[2] as f32).powi(2))
            .sqrt();

        if dist < min_dist {
            min_dist = dist;
            closest_color = color;
        }
    }

    closest_color
}

fn apply_error_diffusion_palettization(img: &mut RgbImage) {
    let width = img.width();
    let height = img.height();
    
    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y); 
            let closest_color = closest_palette_color(pixel);
            
            let error = [
                pixel[0] as f32 - closest_color[0] as f32,
                pixel[1] as f32 - closest_color[1] as f32,
                pixel[2] as f32 - closest_color[2] as f32,
            ];

            if x + 1 < width {
                let mut neighbor = img.get_pixel_mut(x + 1, y);
                for i in 0..3 {
                    neighbor[i] = (neighbor[i] as f32 + error[i] * 0.5).clamp(0.0, 255.0) as u8;
                }
            }
            if y + 1 < height {
                let mut neighbor = img.get_pixel_mut(x, y + 1);
                for i in 0..3 {
                    neighbor[i] = (neighbor[i] as f32 + error[i] * 0.5).clamp(0.0, 255.0) as u8;
                }
            }
            img.put_pixel(x, y, closest_color);
        }
    }
}

fn apply_floyd_steinberg_dithering(img: &mut RgbImage) {
    let width = img.width();
    let height = img.height();
    
    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let luminosity = 0.2126 * pixel[0] as f32 + 0.7152 * pixel[1] as f32 + 0.0722 * pixel[2] as f32;
            let new_pixel_value = if luminosity > 128.0 { 255 } else { 0 };
            let error = luminosity - new_pixel_value as f32;
        
            img.put_pixel(x, y, Rgb([new_pixel_value, new_pixel_value, new_pixel_value]));
            if x + 1 < width {
                let neighbor = img.get_pixel_mut(x + 1, y);
                let pixel_value = &mut neighbor[0];
                *pixel_value = ((*pixel_value as f32) + error * 0.4375) as u8;
            }
            if y + 1 < height {
                let neighbor = img.get_pixel_mut(x, y + 1);
                let pixel_value = &mut neighbor[0];
                *pixel_value = ((*pixel_value as f32) + error * 0.1875) as u8;
                
                if x + 1 < width {
                    let neighbor = img.get_pixel_mut(x + 1, y + 1);
                    let pixel_value = &mut neighbor[0];
                    *pixel_value = ((*pixel_value as f32) + error * 0.3125) as u8;
                }
            }
        }
    }
}

use argh::FromArgs;

#[derive(Debug, Clone, PartialEq, FromArgs)]
/// Convertit une image en monochrome ou vers une palette rÃ©duite de couleurs.
struct DitherArgs {

    /// le fichier dâentrÃ©e
    #[argh(positional)]
    input: String,

    /// le fichier de sortie (optionnel)
    #[argh(positional)]
    output: Option<String>,

    /// le mode dâopÃ©ration
    #[argh(subcommand)]
    mode: Mode
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand)]
enum Mode {
    BlackWhite(OtpsBlackWhite),
    Seuil(OptsSeuil),
    Palette(OptsPalette),
    TramageAlea(OptsTramageAlea),
    BayerMat(OptsBayerMat),
    DiffusionErreur(OptsDiffErreur),
    UnSurDeux(OptsUnSurDeux)
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name="seuil")]
/// Rendu de l'image par seuillage monochrome.
struct OptsSeuil {}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name="monochrome")]
/// Rendu de l'image en deux couleurs.
struct OtpsBlackWhite {}


#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name="palette")]
/// Rendu de l'image avec une palette contenant un nombre limité de couleurs
struct OptsPalette {

    /// le nombre de couleurs à utiliser, dans la liste [NOIR, BLANC, ROUGE, VERT, BLEU, JAUNE, CYAN, MAGENTA]
    #[argh(option, short = 'n', long = "n-couleurs")]
    n_couleurs: usize,
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name="tramage-aleatoire")]
/// Rendu de l'image avec une palette contenant un nombre limité de couleurs
struct OptsTramageAlea {}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name="bayer-mat")]
/// Rendu de l'image avec une palette contenant un nombre limité de couleurs
struct OptsBayerMat {}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name="un-sur-deux")]
/// Rendu de l'image avec un pixel sur deux blanc
struct OptsUnSurDeux{}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name="diffusion-error")]
/// Rendu de l'image avec une palette contenant un nombre limité de couleurs
struct OptsDiffErreur {

    /// la diffusion d'erreur à utiliser, palettisation ou Floyd-Steinberg matrice
    #[argh(option, short = 'd', long = "diff-method")]
    diff_method: String,
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Récupération des arguments de la ligne de commande
    let args: DitherArgs = argh::from_env();
    let path_in = args.input;
    let path_out = args.output.unwrap_or_else(|| "output.png".to_string()); // Fichier de sortie par défaut

    // Chargement de l'image d'entrée
    let img_file = ImageReader::open(&path_in)?;
    let mut img = img_file.decode()?.into_rgb8();


    let luminosity = get_pixel_luminosity(&img, 32, 52);
    println!("La luminosité du pixel (0, 0) est : {}", luminosity);

    // Traitement en fonction du mode spécifié
    match args.mode {
        Mode::UnSurDeux(_) => {
            println!("Application du rendu un pixel sur deux blanc..");
            for (x, y, color) in img.enumerate_pixels_mut() {
                if (x + y) % 2 == 0 {
                    let _color = Rgb([255, 255, 255]);
                    *color = _color;
                }
            }
            img.save(&path_out)?;
        }
        Mode::Seuil(_) => {
            println!("Application du rendu par seuillage monochrome...");
            apply_random_dithering(&mut img); // Appliquer le tramage aléatoire ou un autre type de seuillage
            img.save(&path_out)?;
        }
        Mode::Palette(opts) => {
            println!("Application du rendu avec une palette de {} couleurs...", opts.n_couleurs);
            if opts.n_couleurs == 0{
                for (x, y, color) in img.enumerate_pixels_mut() {
                    let _color = Rgb([0, 0, 0]);
                    *color = _color;
                }
            }
            else{
                let palette = generate_palette(opts.n_couleurs);
                for (_, _, pixel) in img.enumerate_pixels_mut() {
                    *pixel = pixel_to_palette(pixel, &palette);
                }
            }
            img.save(&path_out)?;
        }
        Mode::BlackWhite(_) => {
            println!("Application du rendu en deux couleurs...");
            let light_color = Rgb([152, 152, 5]);
            let dark_color = Rgb([89, 4, 89]);
            for (_, _, pixel) in img.enumerate_pixels_mut() {
                *pixel = pixel_luminosity_to_custom(pixel, light_color, dark_color);
            }
            img.save(&path_out)?;
        },
        Mode::TramageAlea(_) => {
            println!("Application du tramage aléatoire...");
            apply_random_dithering(&mut img);
            img.save(&path_out)?;
        }
        Mode::BayerMat(_) => {
            println!("Application du tramage avec la matrice de Bayer...");
            let img = apply_bayer_dithering(&img, 3);
            img.save(&path_out)?;
        }
        Mode::DiffusionErreur(opts) => {
            println!("Application de la diffusion d'erreur...");
            if opts.diff_method == "palettisation"{
                apply_error_diffusion_palettization(&mut img);
            }
            else if opts.diff_method == "Floyd-Steinberg"{
                apply_floyd_steinberg_dithering(&mut img);
            }
            else if opts.diff_method == "error_diffusion"{
                apply_error_diffusion_dithering(&mut img);
            }
            img.save(&path_out)?;
        }
    }
    println!("Traitement terminé et image sauvegardée sous {}", path_out);
    Ok(())
}

/// Génère une palette de couleurs en fonction du nombre demandé.
/// La palette par défaut contient une sélection de couleurs de base.
fn generate_palette(n: usize) -> Vec<Rgb<u8>> {
    let base_palette = vec![
        Rgb([0, 0, 0]),       // Noir
        Rgb([255, 255, 255]), // Blanc
        Rgb([255, 0, 0]),     // Rouge
        Rgb([0, 255, 0]),     // Vert
        Rgb([0, 0, 255]),     // Bleu
        Rgb([255, 255, 0]),   // Jaune
        Rgb([255, 0, 255]),   // Magenta
        Rgb([0, 255, 255]),   // Cyan
    ];

    // Limiter la palette à 'n' couleurs, en répétant la palette de base si nécessaire
    base_palette.into_iter().cycle().take(n).collect()
}