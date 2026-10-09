pub mod density_function {
    // Auto-generated inline Rust module from RCL
    #[allow(dead_code)]
    #[allow(unused_variables)]
    #[allow(unused_imports)]
    #[allow(warnings)]
    #[allow(clippy::all)]
    
    use super::*;
    
    #[repr(C)]
    pub struct HostConsts {
        pub base_box: BlockBox,
        pub jigsaw_array: [Jigsaw; 500],
        pub piece_array: [Piece; 500],
    }
    
    const minecraft_new_surface_combined_unprocessed_base_spline_coordinates_135471207143960: [f32; 1] = [0_f32];
    const minecraft_new_surface_combined_unprocessed_base_spline_derivatives_135471207143904: [f32; 1] = [1_f32];
    const minecraft_new_surface_combined_unprocessed_base_spline_coordinates_135471207142976: [f32; 1] = [0_f32];
    const minecraft_new_surface_combined_unprocessed_base_spline_derivatives_135471207142920: [f32; 1] = [1_f32];
    const minecraft_new_surface_combined_unprocessed_base_spline_coordinates_135471207141992: [f32; 2] = [0.5_f32, 0.6_f32];
    const minecraft_new_surface_combined_unprocessed_base_spline_derivatives_135471207141936: [f32; 2] = [0_f32, 0_f32];
    const minecraft_new_surface_combined_unprocessed_base_spline_coordinates_135471207141120: [f32; 1] = [0_f32];
    const minecraft_new_surface_combined_unprocessed_base_spline_derivatives_135471207141064: [f32; 1] = [1_f32];
    const minecraft_new_surface_combined_unprocessed_base_spline_coordinates_135471207131520: [f32; 3] = [0.4_f32, 0.5_f32, 0.6_f32];
    const minecraft_new_surface_combined_unprocessed_base_spline_derivatives_135471207131464: [f32; 3] = [0_f32, 0_f32, 0_f32];
    const minecraft_new_surface_combined_unprocessed_base_spline_coordinates_135471207131240: [f32; 1] = [0_f32];
    const minecraft_new_surface_combined_unprocessed_base_spline_derivatives_135471207131184: [f32; 1] = [1_f32];
    const minecraft_new_surface_combined_unprocessed_base_spline_coordinates_135471207127760: [f32; 4] = [0.3_f32, 0.4_f32, 0.5_f32, 0.6_f32];
    const minecraft_new_surface_combined_unprocessed_base_spline_derivatives_135471207127704: [f32; 4] = [0_f32, 0_f32, 0_f32, 0_f32];
    const minecraft_new_surface_combined_unprocessed_base_spline_coordinates_135471207127480: [f32; 1] = [0_f32];
    const minecraft_new_surface_combined_unprocessed_base_spline_derivatives_135471207127424: [f32; 1] = [1_f32];
    const minecraft_new_surface_combined_unprocessed_base_spline_coordinates_135471207120888: [f32; 5] = [0.2_f32, 0.3_f32, 0.4_f32, 0.5_f32, 0.6_f32];
    const minecraft_new_surface_combined_unprocessed_base_spline_derivatives_135471207120832: [f32; 5] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const minecraft_new_surface_combined_unprocessed_base_spline_coordinates_135471207120608: [f32; 1] = [0_f32];
    const minecraft_new_surface_combined_unprocessed_base_spline_derivatives_135471207120552: [f32; 1] = [1_f32];
    const minecraft_new_surface_combined_unprocessed_base_spline_coordinates_135471207114016: [f32; 6] = [0.1_f32, 0.2_f32, 0.3_f32, 0.4_f32, 0.5_f32, 0.6_f32];
    const minecraft_new_surface_combined_unprocessed_base_spline_derivatives_135471207113960: [f32; 6] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const minecraft_new_surface_combined_unprocessed_base_spline_coordinates_135471207113736: [f32; 6] = [0.05_f32, 0.1_f32, 0.15_f32, 0.2_f32, 0.25_f32, 0.3_f32];
    const minecraft_new_surface_combined_unprocessed_base_spline_derivatives_135471207113680: [f32; 6] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const minecraft_new_combination_unfixed_2_spline_coordinates_135471207111568: [f32; 1] = [0_f32];
    const minecraft_new_combination_unfixed_2_spline_derivatives_135471207111512: [f32; 1] = [1_f32];
    const minecraft_new_combination_unfixed_2_spline_coordinates_135471207111232: [f32; 1] = [0_f32];
    const minecraft_new_combination_unfixed_2_spline_derivatives_135471207111176: [f32; 1] = [1_f32];
    const minecraft_new_combination_unfixed_2_spline_coordinates_135471207110896: [f32; 2] = [0.1_f32, 1_f32];
    const minecraft_new_combination_unfixed_2_spline_derivatives_135471207110840: [f32; 2] = [0_f32, 0_f32];
    const minecraft_realism_extreme_hills_slope_height_spline_coordinates_135471207177088: [f32; 1] = [0_f32];
    const minecraft_realism_extreme_hills_slope_height_spline_derivatives_135471207177032: [f32; 1] = [1_f32];
    const minecraft_realism_extreme_hills_slope_height_spline_coordinates_135471207176752: [f32; 1] = [0_f32];
    const minecraft_realism_extreme_hills_slope_height_spline_derivatives_135471207176696: [f32; 1] = [1_f32];
    const minecraft_realism_extreme_hills_slope_height_spline_coordinates_135471207176416: [f32; 2] = [-0.2_f32, 0.2_f32];
    const minecraft_realism_extreme_hills_slope_height_spline_derivatives_135471207176360: [f32; 2] = [0_f32, 0_f32];
    const minecraft_realism_extreme_hills_slope_height_spline_coordinates_135471207176024: [f32; 1] = [0_f32];
    const minecraft_realism_extreme_hills_slope_height_spline_derivatives_135471207175968: [f32; 1] = [0.03_f32];
    const minecraft_realism_extreme_hills_slope_height_spline_coordinates_135471207174728: [f32; 1] = [0_f32];
    const minecraft_realism_extreme_hills_slope_height_spline_derivatives_135471207174672: [f32; 1] = [0.06_f32];
    const minecraft_realism_extreme_hills_slope_height_spline_coordinates_135471207174448: [f32; 2] = [-0.3_f32, 0.3_f32];
    const minecraft_realism_extreme_hills_slope_height_spline_derivatives_135471207174392: [f32; 2] = [0_f32, 0_f32];
    const density_function_Multiply_12_spline_coordinates_135471207186328: [f32; 17] = [0_f32, 0.05_f32, 0.1_f32, 0.2_f32, 0.3_f32, 0.4_f32, 0.5_f32, 0.6_f32, 0.7_f32, 0.8_f32, 0.9_f32, 1_f32, 1.1_f32, 1.2_f32, 1.3_f32, 1.4_f32, 1.5_f32];
    const density_function_Multiply_12_spline_derivatives_135471207186272: [f32; 17] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const minecraft_realism_extreme_mountains_slope_height_spline_coordinates_135471207163856: [f32; 17] = [0_f32, 0.05_f32, 0.1_f32, 0.2_f32, 0.3_f32, 0.4_f32, 0.5_f32, 0.6_f32, 0.7_f32, 0.8_f32, 0.9_f32, 1_f32, 1.1_f32, 1.2_f32, 1.3_f32, 1.4_f32, 1.5_f32];
    const minecraft_realism_extreme_mountains_slope_height_spline_derivatives_135471207163800: [f32; 17] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const minecraft_realism_extreme_mountains_slope_height_spline_coordinates_135471207160328: [f32; 1] = [0_f32];
    const minecraft_realism_extreme_mountains_slope_height_spline_derivatives_135471207160272: [f32; 1] = [0.04_f32];
    const minecraft_realism_extreme_mountains_slope_height_spline_coordinates_135471207159032: [f32; 1] = [0_f32];
    const minecraft_realism_extreme_mountains_slope_height_spline_derivatives_135471207158976: [f32; 1] = [0.05_f32];
    const minecraft_realism_extreme_mountains_slope_height_spline_coordinates_135471207158752: [f32; 2] = [-0.3_f32, 0.3_f32];
    const minecraft_realism_extreme_mountains_slope_height_spline_derivatives_135471207158696: [f32; 2] = [0_f32, 0_f32];
    const density_function_Multiply_48_spline_coordinates_135471207139016: [f32; 17] = [0_f32, 0.05_f32, 0.1_f32, 0.2_f32, 0.3_f32, 0.4_f32, 0.5_f32, 0.6_f32, 0.7_f32, 0.8_f32, 0.9_f32, 1_f32, 1.1_f32, 1.2_f32, 1.3_f32, 1.4_f32, 1.5_f32];
    const density_function_Multiply_48_spline_derivatives_135471207138960: [f32; 17] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const minecraft_caves_new_small_spline_coordinates_135471207101360: [f32; 165] = [-2_f32, -1.975_f32, -1.95_f32, -1.925_f32, -1.9_f32, -1.875_f32, -1.85_f32, -1.825_f32, -1.8_f32, -1.775_f32, -1.75_f32, -1.725_f32, -1.7_f32, -1.675_f32, -1.65_f32, -1.625_f32, -1.6_f32, -1.575_f32, -1.55_f32, -1.525_f32, -1.5_f32, -1.475_f32, -1.45_f32, -1.425_f32, -1.4_f32, -1.375_f32, -1.35_f32, -1.325_f32, -1.3_f32, -1.275_f32, -1.25_f32, -1.225_f32, -1.2_f32, -1.175_f32, -1.15_f32, -1.125_f32, -1.1_f32, -1.075_f32, -1.05_f32, -1.025_f32, -1.1_f32, -1.075_f32, -1.05_f32, -1.025_f32, -1_f32, -0.975_f32, -0.95_f32, -0.925_f32, -0.9_f32, -0.875_f32, -0.85_f32, -0.825_f32, -0.8_f32, -0.775_f32, -0.75_f32, -0.725_f32, -0.7_f32, -0.675_f32, -0.65_f32, -0.625_f32, -0.6_f32, -0.575_f32, -0.55_f32, -0.525_f32, -0.5_f32, -0.475_f32, -0.45_f32, -0.425_f32, -0.4_f32, -0.375_f32, -0.35_f32, -0.325_f32, -0.3_f32, -0.275_f32, -0.25_f32, -0.225_f32, -0.2_f32, -0.175_f32, -0.15_f32, -0.125_f32, -0.1_f32, -0.075_f32, -0.05_f32, -0.025_f32, 0_f32, 0.025_f32, 0.05_f32, 0.075_f32, 0.1_f32, 0.125_f32, 0.15_f32, 0.175_f32, 0.2_f32, 0.225_f32, 0.25_f32, 0.275_f32, 0.3_f32, 0.325_f32, 0.35_f32, 0.375_f32, 0.4_f32, 0.425_f32, 0.45_f32, 0.475_f32, 0.5_f32, 0.525_f32, 0.55_f32, 0.575_f32, 0.6_f32, 0.625_f32, 0.65_f32, 0.675_f32, 0.7_f32, 0.725_f32, 0.75_f32, 0.775_f32, 0.8_f32, 0.825_f32, 0.85_f32, 0.875_f32, 0.9_f32, 0.925_f32, 0.95_f32, 0.975_f32, 1_f32, 1.025_f32, 1.05_f32, 1.075_f32, 1.1_f32, 1.125_f32, 1.15_f32, 1.175_f32, 1.2_f32, 1.225_f32, 1.25_f32, 1.275_f32, 1.3_f32, 1.325_f32, 1.35_f32, 1.375_f32, 1.4_f32, 1.425_f32, 1.45_f32, 1.475_f32, 1.5_f32, 1.525_f32, 1.55_f32, 1.575_f32, 1.6_f32, 1.625_f32, 1.65_f32, 1.675_f32, 1.7_f32, 1.725_f32, 1.75_f32, 1.775_f32, 1.8_f32, 1.825_f32, 1.85_f32, 1.875_f32, 1.9_f32, 1.925_f32, 1.95_f32, 1.975_f32, 2_f32];
    const minecraft_caves_new_small_spline_derivatives_135471207101304: [f32; 165] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const minecraft_realism_mountains_slope_height_spline_coordinates_135471207172000: [f32; 17] = [0_f32, 0.05_f32, 0.1_f32, 0.2_f32, 0.3_f32, 0.4_f32, 0.5_f32, 0.6_f32, 0.7_f32, 0.8_f32, 0.9_f32, 1_f32, 1.1_f32, 1.2_f32, 1.3_f32, 1.4_f32, 1.5_f32];
    const minecraft_realism_mountains_slope_height_spline_derivatives_135471207171944: [f32; 17] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const minecraft_realism_mountains_slope_height_spline_coordinates_135471207168472: [f32; 1] = [0_f32];
    const minecraft_realism_mountains_slope_height_spline_derivatives_135471207168416: [f32; 1] = [0.035_f32];
    const minecraft_realism_mountains_slope_height_spline_coordinates_135471207167176: [f32; 1] = [0_f32];
    const minecraft_realism_mountains_slope_height_spline_derivatives_135471207167120: [f32; 1] = [0.045_f32];
    const minecraft_realism_mountains_slope_height_spline_coordinates_135471207166896: [f32; 2] = [-0.3_f32, 0.3_f32];
    const minecraft_realism_mountains_slope_height_spline_derivatives_135471207166840: [f32; 2] = [0_f32, 0_f32];
    const minecraft_realism_mountains_slope_height_spline_coordinates_135471207126808: [f32; 17] = [0_f32, 0.05_f32, 0.1_f32, 0.2_f32, 0.3_f32, 0.4_f32, 0.5_f32, 0.6_f32, 0.7_f32, 0.8_f32, 0.9_f32, 1_f32, 1.1_f32, 1.2_f32, 1.3_f32, 1.4_f32, 1.5_f32];
    const minecraft_realism_mountains_slope_height_spline_derivatives_135471207126752: [f32; 17] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const minecraft_realism_mountains_slope_height_spline_coordinates_135471207123280: [f32; 1] = [0_f32];
    const minecraft_realism_mountains_slope_height_spline_derivatives_135471207123224: [f32; 1] = [0.035_f32];
    const minecraft_realism_mountains_slope_height_spline_coordinates_135471207122408: [f32; 1] = [0_f32];
    const minecraft_realism_mountains_slope_height_spline_derivatives_135471207122352: [f32; 1] = [0.045_f32];
    const minecraft_realism_mountains_slope_height_spline_coordinates_135471207122128: [f32; 2] = [-0.3_f32, 0.3_f32];
    const minecraft_realism_mountains_slope_height_spline_derivatives_135471207122072: [f32; 2] = [0_f32, 0_f32];
    const minecraft_caves_new_large_spline_coordinates_135471207095760: [f32; 41] = [-2_f32, -1.9_f32, -1.8_f32, -1.7_f32, -1.6_f32, -1.5_f32, -1.4_f32, -1.3_f32, -1.2_f32, -1.1_f32, -1_f32, -0.9_f32, -0.8_f32, -0.7_f32, -0.6_f32, -0.5_f32, -0.4_f32, -0.3_f32, -0.2_f32, -0.1_f32, 0_f32, 0.1_f32, 0.2_f32, 0.3_f32, 0.4_f32, 0.5_f32, 0.6_f32, 0.7_f32, 0.8_f32, 0.9_f32, 1_f32, 1.1_f32, 1.2_f32, 1.3_f32, 1.4_f32, 1.5_f32, 1.6_f32, 1.7_f32, 1.8_f32, 1.9_f32, 2_f32];
    const minecraft_caves_new_large_spline_derivatives_135471207095704: [f32; 41] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const density_function_Multiply_49_spline_coordinates_135471207138456: [f32; 20] = [0_f32, 0.1_f32, 0.2_f32, 0.3_f32, 0.4_f32, 0.5_f32, 0.6_f32, 0.7_f32, 0.8_f32, 0.9_f32, 1_f32, 2_f32, 3_f32, 4_f32, 5_f32, 6_f32, 7_f32, 8_f32, 9_f32, 10_f32];
    const density_function_Multiply_49_spline_derivatives_135471207138400: [f32; 20] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const minecraft_new_surface_height_unprocessed_rivered_spline_coordinates_135471207195368: [f32; 1] = [0_f32];
    const minecraft_new_surface_height_unprocessed_rivered_spline_derivatives_135471207195312: [f32; 1] = [1_f32];
    const minecraft_new_surface_height_unprocessed_rivered_spline_coordinates_135471207193112: [f32; 2] = [0.5_f32, 0.6_f32];
    const minecraft_new_surface_height_unprocessed_rivered_spline_derivatives_135471207193056: [f32; 2] = [0_f32, 0_f32];
    const minecraft_new_surface_height_unprocessed_rivered_spline_coordinates_135471207191816: [f32; 1] = [0_f32];
    const minecraft_new_surface_height_unprocessed_rivered_spline_derivatives_135471207191760: [f32; 1] = [1_f32];
    const minecraft_new_surface_height_unprocessed_rivered_spline_coordinates_135471207189560: [f32; 2] = [0.5_f32, 0.6_f32];
    const minecraft_new_surface_height_unprocessed_rivered_spline_derivatives_135471207189504: [f32; 2] = [0_f32, 0_f32];
    const minecraft_new_surface_height_unprocessed_rivered_spline_coordinates_135471207189280: [f32; 1] = [0_f32];
    const minecraft_new_surface_height_unprocessed_rivered_spline_derivatives_135471207189224: [f32; 1] = [1_f32];
    const minecraft_new_surface_height_unprocessed_rivered_spline_coordinates_135471207177984: [f32; 3] = [0.4_f32, 0.5_f32, 0.6_f32];
    const minecraft_new_surface_height_unprocessed_rivered_spline_derivatives_135471207177928: [f32; 3] = [0_f32, 0_f32, 0_f32];
    const minecraft_new_surface_height_unprocessed_rivered_spline_coordinates_135471207177704: [f32; 1] = [0_f32];
    const minecraft_new_surface_height_unprocessed_rivered_spline_derivatives_135471207177648: [f32; 1] = [1_f32];
    const minecraft_new_surface_height_unprocessed_rivered_spline_coordinates_135471207173800: [f32; 4] = [0.3_f32, 0.4_f32, 0.5_f32, 0.6_f32];
    const minecraft_new_surface_height_unprocessed_rivered_spline_derivatives_135471207173744: [f32; 4] = [0_f32, 0_f32, 0_f32, 0_f32];
    const minecraft_new_surface_height_unprocessed_rivered_spline_coordinates_135471207173520: [f32; 1] = [0_f32];
    const minecraft_new_surface_height_unprocessed_rivered_spline_derivatives_135471207173464: [f32; 1] = [1_f32];
    const minecraft_new_surface_height_unprocessed_rivered_spline_coordinates_135471207165656: [f32; 5] = [0.2_f32, 0.3_f32, 0.4_f32, 0.5_f32, 0.6_f32];
    const minecraft_new_surface_height_unprocessed_rivered_spline_derivatives_135471207165600: [f32; 5] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const minecraft_new_surface_height_unprocessed_rivered_spline_coordinates_135471207165376: [f32; 1] = [0_f32];
    const minecraft_new_surface_height_unprocessed_rivered_spline_derivatives_135471207165320: [f32; 1] = [1_f32];
    const minecraft_new_surface_height_unprocessed_rivered_spline_coordinates_135471207157512: [f32; 6] = [0.1_f32, 0.2_f32, 0.3_f32, 0.4_f32, 0.5_f32, 0.6_f32];
    const minecraft_new_surface_height_unprocessed_rivered_spline_derivatives_135471207157456: [f32; 6] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const minecraft_new_surface_height_unprocessed_rivered_spline_coordinates_135471207157232: [f32; 7] = [0_f32, 0.05_f32, 0.1_f32, 0.15_f32, 0.2_f32, 0.25_f32, 0.3_f32];
    const minecraft_new_surface_height_unprocessed_rivered_spline_derivatives_135471207157176: [f32; 7] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const minecraft_realism_extreme_hills_slope_height_spline_coordinates_135471207130624: [f32; 1] = [0_f32];
    const minecraft_realism_extreme_hills_slope_height_spline_derivatives_135471207130568: [f32; 1] = [1_f32];
    const minecraft_realism_extreme_hills_slope_height_spline_coordinates_135471207130288: [f32; 1] = [0_f32];
    const minecraft_realism_extreme_hills_slope_height_spline_derivatives_135471207130232: [f32; 1] = [1_f32];
    const minecraft_realism_extreme_hills_slope_height_spline_coordinates_135471207129952: [f32; 2] = [-0.2_f32, 0.2_f32];
    const minecraft_realism_extreme_hills_slope_height_spline_derivatives_135471207129896: [f32; 2] = [0_f32, 0_f32];
    const minecraft_realism_extreme_hills_slope_height_spline_coordinates_135471207129560: [f32; 1] = [0_f32];
    const minecraft_realism_extreme_hills_slope_height_spline_derivatives_135471207129504: [f32; 1] = [0.03_f32];
    const minecraft_realism_extreme_hills_slope_height_spline_coordinates_135471207128688: [f32; 1] = [0_f32];
    const minecraft_realism_extreme_hills_slope_height_spline_derivatives_135471207128632: [f32; 1] = [0.06_f32];
    const minecraft_realism_extreme_hills_slope_height_spline_coordinates_135471207128408: [f32; 2] = [-0.3_f32, 0.3_f32];
    const minecraft_realism_extreme_hills_slope_height_spline_derivatives_135471207128352: [f32; 2] = [0_f32, 0_f32];
    const minecraft_caves_new_medium_spline_coordinates_135471207097840: [f32; 81] = [-2_f32, -1.95_f32, -1.9_f32, -1.85_f32, -1.8_f32, -1.75_f32, -1.7_f32, -1.65_f32, -1.6_f32, -1.55_f32, -1.5_f32, -1.45_f32, -1.4_f32, -1.35_f32, -1.3_f32, -1.25_f32, -1.2_f32, -1.15_f32, -1.1_f32, -1.05_f32, -1_f32, -0.95_f32, -0.9_f32, -0.85_f32, -0.8_f32, -0.75_f32, -0.7_f32, -0.65_f32, -0.6_f32, -0.55_f32, -0.5_f32, -0.45_f32, -0.4_f32, -0.35_f32, -0.3_f32, -0.25_f32, -0.2_f32, -0.15_f32, -0.1_f32, -0.05_f32, 0_f32, 0.05_f32, 0.1_f32, 0.15_f32, 0.2_f32, 0.25_f32, 0.3_f32, 0.35_f32, 0.4_f32, 0.45_f32, 0.5_f32, 0.55_f32, 0.6_f32, 0.65_f32, 0.7_f32, 0.75_f32, 0.8_f32, 0.85_f32, 0.9_f32, 0.95_f32, 1_f32, 1.05_f32, 1.1_f32, 1.15_f32, 1.2_f32, 1.25_f32, 1.3_f32, 1.35_f32, 1.4_f32, 1.45_f32, 1.5_f32, 1.55_f32, 1.6_f32, 1.65_f32, 1.7_f32, 1.75_f32, 1.8_f32, 1.85_f32, 1.9_f32, 1.95_f32, 2_f32];
    const minecraft_caves_new_medium_spline_derivatives_135471207097784: [f32; 81] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const minecraft_realism_hills_slope_height_spline_coordinates_135471207140504: [f32; 1] = [0_f32];
    const minecraft_realism_hills_slope_height_spline_derivatives_135471207140448: [f32; 1] = [1_f32];
    const minecraft_realism_hills_slope_height_spline_coordinates_135471207139632: [f32; 1] = [0_f32];
    const minecraft_realism_hills_slope_height_spline_derivatives_135471207139576: [f32; 1] = [1_f32];
    const minecraft_realism_hills_slope_height_spline_coordinates_135471207134304: [f32; 2] = [-0.2_f32, 0.2_f32];
    const minecraft_realism_hills_slope_height_spline_derivatives_135471207134248: [f32; 2] = [0_f32, 0_f32];
    const minecraft_realism_hills_slope_height_spline_coordinates_135471207133912: [f32; 1] = [0_f32];
    const minecraft_realism_hills_slope_height_spline_derivatives_135471207133856: [f32; 1] = [0.03_f32];
    const minecraft_realism_hills_slope_height_spline_coordinates_135471207133040: [f32; 1] = [0_f32];
    const minecraft_realism_hills_slope_height_spline_derivatives_135471207132984: [f32; 1] = [0.06_f32];
    const minecraft_realism_hills_slope_height_spline_coordinates_135471207132760: [f32; 2] = [-0.3_f32, 0.3_f32];
    const minecraft_realism_hills_slope_height_spline_derivatives_135471207132704: [f32; 2] = [0_f32, 0_f32];
    const minecraft_new_surface_height_unprocessed_hollowed_spline_coordinates_135471207195928: [f32; 1] = [0_f32];
    const minecraft_new_surface_height_unprocessed_hollowed_spline_derivatives_135471207195872: [f32; 1] = [1_f32];
    const minecraft_new_surface_height_unprocessed_hollowed_spline_coordinates_135471207155568: [f32; 1] = [0_f32];
    const minecraft_new_surface_height_unprocessed_hollowed_spline_derivatives_135471207155512: [f32; 1] = [1_f32];
    const minecraft_new_surface_height_unprocessed_hollowed_spline_coordinates_135471207146352: [f32; 2] = [0.6_f32, 0.8_f32];
    const minecraft_new_surface_height_unprocessed_hollowed_spline_derivatives_135471207146296: [f32; 2] = [0_f32, 0_f32];
    const minecraft_new_combination_unfixed_3_spline_coordinates_135471207196880: [f32; 1] = [0_f32];
    const minecraft_new_combination_unfixed_3_spline_derivatives_135471207196824: [f32; 1] = [1_f32];
    const minecraft_new_combination_unfixed_3_spline_coordinates_135471207145504: [f32; 1] = [0_f32];
    const minecraft_new_combination_unfixed_3_spline_derivatives_135471207145448: [f32; 1] = [1_f32];
    const minecraft_new_combination_unfixed_3_spline_coordinates_135471207144576: [f32; 2] = [0.7_f32, 1.5_f32];
    const minecraft_new_combination_unfixed_3_spline_derivatives_135471207144520: [f32; 2] = [0_f32, 0_f32];
    const minecraft_new_combination_unfixed_new_spline_coordinates_135471207197440: [f32; 1] = [0_f32];
    const minecraft_new_combination_unfixed_new_spline_derivatives_135471207197384: [f32; 1] = [1_f32];
    const minecraft_new_combination_unfixed_new_spline_coordinates_135471207112128: [f32; 1] = [0_f32];
    const minecraft_new_combination_unfixed_new_spline_derivatives_135471207112072: [f32; 1] = [1_f32];
    const minecraft_new_combination_unfixed_new_spline_coordinates_135471207110192: [f32; 2] = [0_f32, 2_f32];
    const minecraft_new_combination_unfixed_new_spline_derivatives_135471207110136: [f32; 2] = [0_f32, 0_f32];
    const minecraft_new_combination_unfixed_new_spline_coordinates_135471207109208: [f32; 1] = [0_f32];
    const minecraft_new_combination_unfixed_new_spline_derivatives_135471207109152: [f32; 1] = [1_f32];
    const minecraft_new_combination_unfixed_new_spline_coordinates_135471207108928: [f32; 1] = [0_f32];
    const minecraft_new_combination_unfixed_new_spline_derivatives_135471207108872: [f32; 1] = [1_f32];
    const minecraft_new_combination_unfixed_new_spline_coordinates_135471207107632: [f32; 2] = [0_f32, 0.5_f32];
    const minecraft_new_combination_unfixed_new_spline_derivatives_135471207107576: [f32; 2] = [0_f32, 0_f32];
    const minecraft_new_combination_unfixed_new_spline_coordinates_135471207106336: [f32; 1] = [0_f32];
    const minecraft_new_combination_unfixed_new_spline_derivatives_135471207106280: [f32; 1] = [1_f32];
    const minecraft_new_combination_unfixed_new_spline_coordinates_135471207105040: [f32; 2] = [0_f32, 0.5_f32];
    const minecraft_new_combination_unfixed_new_spline_derivatives_135471207104984: [f32; 2] = [0_f32, 0_f32];
    const minecraft_new_combination_unfixed_new_spline_coordinates_135471207104760: [f32; 1] = [0_f32];
    const minecraft_new_combination_unfixed_new_spline_derivatives_135471207104704: [f32; 1] = [1_f32];
    const minecraft_new_combination_unfixed_new_spline_coordinates_135471207103464: [f32; 2] = [0_f32, 0.5_f32];
    const minecraft_new_combination_unfixed_new_spline_derivatives_135471207103408: [f32; 2] = [0_f32, 0_f32];
    const minecraft_new_combination_unfixed_new_spline_coordinates_135471207103184: [f32; 1] = [0_f32];
    const minecraft_new_combination_unfixed_new_spline_derivatives_135471207103128: [f32; 1] = [1_f32];
    const minecraft_new_combination_unfixed_new_spline_coordinates_135471207102848: [f32; 4] = [0.1_f32, 0.35_f32, 0.7_f32, 1_f32];
    const minecraft_new_combination_unfixed_new_spline_derivatives_135471207102792: [f32; 4] = [0_f32, 0_f32, 0_f32, 0_f32];
    const minecraft_new_combination_unfixed_new_spline_coordinates_135471207101920: [f32; 1] = [0_f32];
    const minecraft_new_combination_unfixed_new_spline_derivatives_135471207101864: [f32; 1] = [1_f32];
    const minecraft_new_combination_unfixed_new_spline_coordinates_135471207098680: [f32; 2] = [0_f32, 0.5_f32];
    const minecraft_new_combination_unfixed_new_spline_derivatives_135471207098624: [f32; 2] = [0_f32, 0_f32];
    const minecraft_new_combination_unfixed_new_spline_coordinates_135471207098400: [f32; 1] = [0_f32];
    const minecraft_new_combination_unfixed_new_spline_derivatives_135471207098344: [f32; 1] = [1_f32];
    const minecraft_new_combination_unfixed_new_spline_coordinates_135471207096600: [f32; 2] = [0_f32, 0.5_f32];
    const minecraft_new_combination_unfixed_new_spline_derivatives_135471207096544: [f32; 2] = [0_f32, 0_f32];
    const minecraft_new_combination_unfixed_new_spline_coordinates_135471207096320: [f32; 1] = [0_f32];
    const minecraft_new_combination_unfixed_new_spline_derivatives_135471207096264: [f32; 1] = [1_f32];
    const minecraft_new_combination_unfixed_new_spline_coordinates_135471207094520: [f32; 2] = [0_f32, 0.5_f32];
    const minecraft_new_combination_unfixed_new_spline_derivatives_135471207094464: [f32; 2] = [0_f32, 0_f32];
    const minecraft_new_combination_unfixed_new_spline_coordinates_135471207094240: [f32; 4] = [0.1_f32, 0.35_f32, 0.7_f32, 1_f32];
    const minecraft_new_combination_unfixed_new_spline_derivatives_135471207094184: [f32; 4] = [0_f32, 0_f32, 0_f32, 0_f32];
    const minecraft_new_combination_unfixed_new_spline_coordinates_135471207093960: [f32; 2] = [-1_f32, 1_f32];
    const minecraft_new_combination_unfixed_new_spline_derivatives_135471207093904: [f32; 2] = [0_f32, 0_f32];
    const minecraft_new_combination_unfixed_new_spline_coordinates_135471207092664: [f32; 2] = [0.02_f32, 0.05_f32];
    const minecraft_new_combination_unfixed_new_spline_derivatives_135471207092608: [f32; 2] = [0_f32, 0_f32];
    const minecraft_realism_extreme_mountains_slope_height_spline_coordinates_135471207119936: [f32; 17] = [0_f32, 0.05_f32, 0.1_f32, 0.2_f32, 0.3_f32, 0.4_f32, 0.5_f32, 0.6_f32, 0.7_f32, 0.8_f32, 0.9_f32, 1_f32, 1.1_f32, 1.2_f32, 1.3_f32, 1.4_f32, 1.5_f32];
    const minecraft_realism_extreme_mountains_slope_height_spline_derivatives_135471207119880: [f32; 17] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const minecraft_realism_extreme_mountains_slope_height_spline_coordinates_135471207116408: [f32; 1] = [0_f32];
    const minecraft_realism_extreme_mountains_slope_height_spline_derivatives_135471207116352: [f32; 1] = [0.04_f32];
    const minecraft_realism_extreme_mountains_slope_height_spline_coordinates_135471207115536: [f32; 1] = [0_f32];
    const minecraft_realism_extreme_mountains_slope_height_spline_derivatives_135471207115480: [f32; 1] = [0.05_f32];
    const minecraft_realism_extreme_mountains_slope_height_spline_coordinates_135471207115256: [f32; 2] = [-0.3_f32, 0.3_f32];
    const minecraft_realism_extreme_mountains_slope_height_spline_derivatives_135471207115200: [f32; 2] = [0_f32, 0_f32];
    const density_function_Add_40_spline_coordinates_135471207155008: [f32; 1] = [0_f32];
    const density_function_Add_40_spline_derivatives_135471207154952: [f32; 1] = [1_f32];
    const density_function_Add_40_spline_coordinates_135471207154672: [f32; 2] = [0.5_f32, 0.6_f32];
    const density_function_Add_40_spline_derivatives_135471207154616: [f32; 2] = [0_f32, 0_f32];
    const density_function_Add_40_spline_coordinates_135471207154336: [f32; 1] = [0_f32];
    const density_function_Add_40_spline_derivatives_135471207154280: [f32; 1] = [1_f32];
    const density_function_Add_40_spline_coordinates_135471207154000: [f32; 2] = [0.5_f32, 0.6_f32];
    const density_function_Add_40_spline_derivatives_135471207153944: [f32; 2] = [0_f32, 0_f32];
    const density_function_Add_40_spline_coordinates_135471207153720: [f32; 1] = [0_f32];
    const density_function_Add_40_spline_derivatives_135471207153664: [f32; 1] = [1_f32];
    const density_function_Add_40_spline_coordinates_135471207153384: [f32; 3] = [0.4_f32, 0.5_f32, 0.6_f32];
    const density_function_Add_40_spline_derivatives_135471207153328: [f32; 3] = [0_f32, 0_f32, 0_f32];
    const density_function_Add_40_spline_coordinates_135471207153104: [f32; 1] = [0_f32];
    const density_function_Add_40_spline_derivatives_135471207153048: [f32; 1] = [1_f32];
    const density_function_Add_40_spline_coordinates_135471207152768: [f32; 4] = [0.3_f32, 0.4_f32, 0.5_f32, 0.6_f32];
    const density_function_Add_40_spline_derivatives_135471207152712: [f32; 4] = [0_f32, 0_f32, 0_f32, 0_f32];
    const density_function_Add_40_spline_coordinates_135471207152488: [f32; 1] = [0_f32];
    const density_function_Add_40_spline_derivatives_135471207152432: [f32; 1] = [1_f32];
    const density_function_Add_40_spline_coordinates_135471207152152: [f32; 5] = [0.2_f32, 0.3_f32, 0.4_f32, 0.5_f32, 0.6_f32];
    const density_function_Add_40_spline_derivatives_135471207152096: [f32; 5] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const density_function_Add_40_spline_coordinates_135471207151872: [f32; 1] = [0_f32];
    const density_function_Add_40_spline_derivatives_135471207151816: [f32; 1] = [1_f32];
    const density_function_Add_40_spline_coordinates_135471207151536: [f32; 6] = [0.1_f32, 0.2_f32, 0.3_f32, 0.4_f32, 0.5_f32, 0.6_f32];
    const density_function_Add_40_spline_derivatives_135471207151480: [f32; 6] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const density_function_Add_40_spline_coordinates_135471207151256: [f32; 7] = [0_f32, 0.05_f32, 0.1_f32, 0.15_f32, 0.2_f32, 0.25_f32, 0.3_f32];
    const density_function_Add_40_spline_derivatives_135471207151200: [f32; 7] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const density_function_Add_40_spline_coordinates_135471207150808: [f32; 1] = [0_f32];
    const density_function_Add_40_spline_derivatives_135471207150752: [f32; 1] = [1_f32];
    const density_function_Add_40_spline_coordinates_135471207150528: [f32; 2] = [0.5_f32, 0.6_f32];
    const density_function_Add_40_spline_derivatives_135471207150472: [f32; 2] = [0_f32, 0_f32];
    const density_function_Add_40_spline_coordinates_135471207150248: [f32; 1] = [0_f32];
    const density_function_Add_40_spline_derivatives_135471207150192: [f32; 1] = [1_f32];
    const density_function_Add_40_spline_coordinates_135471207149968: [f32; 2] = [0.5_f32, 0.6_f32];
    const density_function_Add_40_spline_derivatives_135471207149912: [f32; 2] = [0_f32, 0_f32];
    const density_function_Add_40_spline_coordinates_135471207149688: [f32; 1] = [0_f32];
    const density_function_Add_40_spline_derivatives_135471207149632: [f32; 1] = [1_f32];
    const density_function_Add_40_spline_coordinates_135471207149408: [f32; 3] = [0.4_f32, 0.5_f32, 0.6_f32];
    const density_function_Add_40_spline_derivatives_135471207149352: [f32; 3] = [0_f32, 0_f32, 0_f32];
    const density_function_Add_40_spline_coordinates_135471207149128: [f32; 1] = [0_f32];
    const density_function_Add_40_spline_derivatives_135471207149072: [f32; 1] = [1_f32];
    const density_function_Add_40_spline_coordinates_135471207148848: [f32; 4] = [0.3_f32, 0.4_f32, 0.5_f32, 0.6_f32];
    const density_function_Add_40_spline_derivatives_135471207148792: [f32; 4] = [0_f32, 0_f32, 0_f32, 0_f32];
    const density_function_Add_40_spline_coordinates_135471207148568: [f32; 1] = [0_f32];
    const density_function_Add_40_spline_derivatives_135471207148512: [f32; 1] = [1_f32];
    const density_function_Add_40_spline_coordinates_135471207148288: [f32; 5] = [0.2_f32, 0.3_f32, 0.4_f32, 0.5_f32, 0.6_f32];
    const density_function_Add_40_spline_derivatives_135471207148232: [f32; 5] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const density_function_Add_40_spline_coordinates_135471207148008: [f32; 1] = [0_f32];
    const density_function_Add_40_spline_derivatives_135471207147952: [f32; 1] = [1_f32];
    const density_function_Add_40_spline_coordinates_135471207147728: [f32; 6] = [0.1_f32, 0.2_f32, 0.3_f32, 0.4_f32, 0.5_f32, 0.6_f32];
    const density_function_Add_40_spline_derivatives_135471207147672: [f32; 6] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const density_function_Add_40_spline_coordinates_135471207147448: [f32; 7] = [0_f32, 0.05_f32, 0.1_f32, 0.15_f32, 0.2_f32, 0.25_f32, 0.3_f32];
    const density_function_Add_40_spline_derivatives_135471207147392: [f32; 7] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const density_function_Add_40_spline_coordinates_135471207147056: [f32; 2] = [0.8_f32, 2_f32];
    const density_function_Add_40_spline_derivatives_135471207147000: [f32; 2] = [0_f32, 0_f32];
    const minecraft_new_combination_fixed_spline_coordinates_135471207198000: [f32; 1] = [0_f32];
    const minecraft_new_combination_fixed_spline_derivatives_135471207197944: [f32; 1] = [1_f32];
    const minecraft_new_combination_fixed_spline_coordinates_135471207091960: [f32; 2] = [-63_f32, -54_f32];
    const minecraft_new_combination_fixed_spline_derivatives_135471207091904: [f32; 2] = [0_f32, 0_f32];
    const density_function_Multiply_13_spline_coordinates_135471207185768: [f32; 20] = [0_f32, 0.1_f32, 0.2_f32, 0.3_f32, 0.4_f32, 0.5_f32, 0.6_f32, 0.7_f32, 0.8_f32, 0.9_f32, 1_f32, 2_f32, 3_f32, 4_f32, 5_f32, 6_f32, 7_f32, 8_f32, 9_f32, 10_f32];
    const density_function_Multiply_13_spline_derivatives_135471207185712: [f32; 20] = [0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
    const minecraft_realism_hills_slope_height_spline_coordinates_135471207188240: [f32; 1] = [0_f32];
    const minecraft_realism_hills_slope_height_spline_derivatives_135471207188184: [f32; 1] = [1_f32];
    const minecraft_realism_hills_slope_height_spline_coordinates_135471207186944: [f32; 1] = [0_f32];
    const minecraft_realism_hills_slope_height_spline_derivatives_135471207186888: [f32; 1] = [1_f32];
    const minecraft_realism_hills_slope_height_spline_coordinates_135471207181616: [f32; 2] = [-0.2_f32, 0.2_f32];
    const minecraft_realism_hills_slope_height_spline_derivatives_135471207181560: [f32; 2] = [0_f32, 0_f32];
    const minecraft_realism_hills_slope_height_spline_coordinates_135471207181224: [f32; 1] = [0_f32];
    const minecraft_realism_hills_slope_height_spline_derivatives_135471207181168: [f32; 1] = [0.03_f32];
    const minecraft_realism_hills_slope_height_spline_coordinates_135471207179928: [f32; 1] = [0_f32];
    const minecraft_realism_hills_slope_height_spline_derivatives_135471207179872: [f32; 1] = [0.06_f32];
    const minecraft_realism_hills_slope_height_spline_coordinates_135471207179648: [f32; 2] = [-0.3_f32, 0.3_f32];
    const minecraft_realism_hills_slope_height_spline_derivatives_135471207179592: [f32; 2] = [0_f32, 0_f32];
    fn density_function_ShiftedNoise_27_d5x1x5_minecraft_gravel_shifted_28(rpos3: Vec3, perm_table_minecraft_gravel_0_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__5: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__5: &PerlinNoiseSampler, host_consts: &HostConsts) -> f64 {
        let mut n: f64;
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), perm_table_minecraft_gravel_0_octave__8) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), perm_table_minecraft_gravel_1_octave__8) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.0078125_f64), perm_table_minecraft_gravel_0_octave__7) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), perm_table_minecraft_gravel_1_octave__7) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.015625_f64), perm_table_minecraft_gravel_0_octave__6) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.015908232628398793_f64), perm_table_minecraft_gravel_1_octave__6) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.03125_f64), perm_table_minecraft_gravel_0_octave__5) * 0.06666666666666667_f64));
        n = (n + (perlin((rpos3 * 0.031816465256797585_f64), perm_table_minecraft_gravel_1_octave__5) * 0.06666666666666667_f64));
        return (n * 1.3333333333333333_f64);
    }
    
    fn density_function_ShiftedNoise_35_d5x1x5_minecraft_gravel_shifted_36(rpos3: Vec3, perm_table_minecraft_gravel_0_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__5: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__5: &PerlinNoiseSampler, host_consts: &HostConsts) -> f64 {
        let mut n: f64;
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), perm_table_minecraft_gravel_0_octave__8) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), perm_table_minecraft_gravel_1_octave__8) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.0078125_f64), perm_table_minecraft_gravel_0_octave__7) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), perm_table_minecraft_gravel_1_octave__7) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.015625_f64), perm_table_minecraft_gravel_0_octave__6) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.015908232628398793_f64), perm_table_minecraft_gravel_1_octave__6) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.03125_f64), perm_table_minecraft_gravel_0_octave__5) * 0.06666666666666667_f64));
        n = (n + (perlin((rpos3 * 0.031816465256797585_f64), perm_table_minecraft_gravel_1_octave__5) * 0.06666666666666667_f64));
        return (n * 1.3333333333333333_f64);
    }
    
    fn density_function_ShiftedNoise_61_d5x263x5_minecraft_gravel_shifted_62(rpos3: Vec3, perm_table_minecraft_gravel_0_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__5: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__5: &PerlinNoiseSampler, host_consts: &HostConsts) -> f64 {
        let mut n: f64;
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), perm_table_minecraft_gravel_0_octave__8) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), perm_table_minecraft_gravel_1_octave__8) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.0078125_f64), perm_table_minecraft_gravel_0_octave__7) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), perm_table_minecraft_gravel_1_octave__7) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.015625_f64), perm_table_minecraft_gravel_0_octave__6) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.015908232628398793_f64), perm_table_minecraft_gravel_1_octave__6) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.03125_f64), perm_table_minecraft_gravel_0_octave__5) * 0.06666666666666667_f64));
        n = (n + (perlin((rpos3 * 0.031816465256797585_f64), perm_table_minecraft_gravel_1_octave__5) * 0.06666666666666667_f64));
        return (n * 1.3333333333333333_f64);
    }
    
    fn density_function_ShiftedNoise_52_d5x263x5_minecraft_gravel_shifted_53(rpos3: Vec3, perm_table_minecraft_gravel_0_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__5: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__5: &PerlinNoiseSampler, host_consts: &HostConsts) -> f64 {
        let mut n: f64;
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), perm_table_minecraft_gravel_0_octave__8) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), perm_table_minecraft_gravel_1_octave__8) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.0078125_f64), perm_table_minecraft_gravel_0_octave__7) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), perm_table_minecraft_gravel_1_octave__7) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.015625_f64), perm_table_minecraft_gravel_0_octave__6) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.015908232628398793_f64), perm_table_minecraft_gravel_1_octave__6) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.03125_f64), perm_table_minecraft_gravel_0_octave__5) * 0.06666666666666667_f64));
        n = (n + (perlin((rpos3 * 0.031816465256797585_f64), perm_table_minecraft_gravel_1_octave__5) * 0.06666666666666667_f64));
        return (n * 1.3333333333333333_f64);
    }
    
    fn density_function_ShiftedNoise_50_d5x263x5_minecraft_gravel_shifted_51(rpos3: Vec3, perm_table_minecraft_gravel_0_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__5: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__5: &PerlinNoiseSampler, host_consts: &HostConsts) -> f64 {
        let mut n: f64;
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), perm_table_minecraft_gravel_0_octave__8) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), perm_table_minecraft_gravel_1_octave__8) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.0078125_f64), perm_table_minecraft_gravel_0_octave__7) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), perm_table_minecraft_gravel_1_octave__7) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.015625_f64), perm_table_minecraft_gravel_0_octave__6) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.015908232628398793_f64), perm_table_minecraft_gravel_1_octave__6) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.03125_f64), perm_table_minecraft_gravel_0_octave__5) * 0.06666666666666667_f64));
        n = (n + (perlin((rpos3 * 0.031816465256797585_f64), perm_table_minecraft_gravel_1_octave__5) * 0.06666666666666667_f64));
        return (n * 1.3333333333333333_f64);
    }
    
    fn density_function_ShiftedNoise_25_d5x1x5_minecraft_gravel_shifted_26(rpos3: Vec3, perm_table_minecraft_gravel_0_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__5: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__5: &PerlinNoiseSampler, host_consts: &HostConsts) -> f64 {
        let mut n: f64;
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), perm_table_minecraft_gravel_0_octave__8) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), perm_table_minecraft_gravel_1_octave__8) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.0078125_f64), perm_table_minecraft_gravel_0_octave__7) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), perm_table_minecraft_gravel_1_octave__7) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.015625_f64), perm_table_minecraft_gravel_0_octave__6) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.015908232628398793_f64), perm_table_minecraft_gravel_1_octave__6) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.03125_f64), perm_table_minecraft_gravel_0_octave__5) * 0.06666666666666667_f64));
        n = (n + (perlin((rpos3 * 0.031816465256797585_f64), perm_table_minecraft_gravel_1_octave__5) * 0.06666666666666667_f64));
        return (n * 1.3333333333333333_f64);
    }
    
    fn density_function_ShiftedNoise_56_d5x263x5_minecraft_gravel_shifted_57(rpos3: Vec3, perm_table_minecraft_gravel_0_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__5: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__5: &PerlinNoiseSampler, host_consts: &HostConsts) -> f64 {
        let mut n: f64;
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), perm_table_minecraft_gravel_0_octave__8) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), perm_table_minecraft_gravel_1_octave__8) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.0078125_f64), perm_table_minecraft_gravel_0_octave__7) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), perm_table_minecraft_gravel_1_octave__7) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.015625_f64), perm_table_minecraft_gravel_0_octave__6) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.015908232628398793_f64), perm_table_minecraft_gravel_1_octave__6) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.03125_f64), perm_table_minecraft_gravel_0_octave__5) * 0.06666666666666667_f64));
        n = (n + (perlin((rpos3 * 0.031816465256797585_f64), perm_table_minecraft_gravel_1_octave__5) * 0.06666666666666667_f64));
        return (n * 1.3333333333333333_f64);
    }
    
    fn density_function_ShiftedNoise_63_d5x263x5_minecraft_gravel_shifted_64(rpos3: Vec3, perm_table_minecraft_gravel_0_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__5: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__5: &PerlinNoiseSampler, host_consts: &HostConsts) -> f64 {
        let mut n: f64;
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), perm_table_minecraft_gravel_0_octave__8) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), perm_table_minecraft_gravel_1_octave__8) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.0078125_f64), perm_table_minecraft_gravel_0_octave__7) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), perm_table_minecraft_gravel_1_octave__7) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.015625_f64), perm_table_minecraft_gravel_0_octave__6) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.015908232628398793_f64), perm_table_minecraft_gravel_1_octave__6) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.03125_f64), perm_table_minecraft_gravel_0_octave__5) * 0.06666666666666667_f64));
        n = (n + (perlin((rpos3 * 0.031816465256797585_f64), perm_table_minecraft_gravel_1_octave__5) * 0.06666666666666667_f64));
        return (n * 1.3333333333333333_f64);
    }
    
    fn density_function_ShiftedNoise_16_d5x1x5_minecraft_gravel_shifted_17(rpos3: Vec3, perm_table_minecraft_gravel_0_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__5: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__5: &PerlinNoiseSampler, host_consts: &HostConsts) -> f64 {
        let mut n: f64;
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), perm_table_minecraft_gravel_0_octave__8) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), perm_table_minecraft_gravel_1_octave__8) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.0078125_f64), perm_table_minecraft_gravel_0_octave__7) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), perm_table_minecraft_gravel_1_octave__7) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.015625_f64), perm_table_minecraft_gravel_0_octave__6) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.015908232628398793_f64), perm_table_minecraft_gravel_1_octave__6) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.03125_f64), perm_table_minecraft_gravel_0_octave__5) * 0.06666666666666667_f64));
        n = (n + (perlin((rpos3 * 0.031816465256797585_f64), perm_table_minecraft_gravel_1_octave__5) * 0.06666666666666667_f64));
        return (n * 1.3333333333333333_f64);
    }
    
    fn density_function_ShiftedNoise_58_d5x263x5_minecraft_gravel_shifted_59(rpos3: Vec3, perm_table_minecraft_gravel_0_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__5: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__5: &PerlinNoiseSampler, host_consts: &HostConsts) -> f64 {
        let mut n: f64;
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), perm_table_minecraft_gravel_0_octave__8) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), perm_table_minecraft_gravel_1_octave__8) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.0078125_f64), perm_table_minecraft_gravel_0_octave__7) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), perm_table_minecraft_gravel_1_octave__7) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.015625_f64), perm_table_minecraft_gravel_0_octave__6) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.015908232628398793_f64), perm_table_minecraft_gravel_1_octave__6) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.03125_f64), perm_table_minecraft_gravel_0_octave__5) * 0.06666666666666667_f64));
        n = (n + (perlin((rpos3 * 0.031816465256797585_f64), perm_table_minecraft_gravel_1_octave__5) * 0.06666666666666667_f64));
        return (n * 1.3333333333333333_f64);
    }
    
    fn density_function_ShiftedNoise_14_d5x1x5_minecraft_gravel_shifted_15(rpos3: Vec3, perm_table_minecraft_gravel_0_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__5: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__5: &PerlinNoiseSampler, host_consts: &HostConsts) -> f64 {
        let mut n: f64;
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), perm_table_minecraft_gravel_0_octave__8) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), perm_table_minecraft_gravel_1_octave__8) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.0078125_f64), perm_table_minecraft_gravel_0_octave__7) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), perm_table_minecraft_gravel_1_octave__7) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.015625_f64), perm_table_minecraft_gravel_0_octave__6) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.015908232628398793_f64), perm_table_minecraft_gravel_1_octave__6) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.03125_f64), perm_table_minecraft_gravel_0_octave__5) * 0.06666666666666667_f64));
        n = (n + (perlin((rpos3 * 0.031816465256797585_f64), perm_table_minecraft_gravel_1_octave__5) * 0.06666666666666667_f64));
        return (n * 1.3333333333333333_f64);
    }
    
    fn density_function_ShiftedNoise_33_d5x1x5_minecraft_gravel_shifted_34(rpos3: Vec3, perm_table_minecraft_gravel_0_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__5: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__5: &PerlinNoiseSampler, host_consts: &HostConsts) -> f64 {
        let mut n: f64;
        n = 0_f64;
        n = (n + (perlin((rpos3 * 0.00390625_f64), perm_table_minecraft_gravel_0_octave__8) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.003977058157099698_f64), perm_table_minecraft_gravel_1_octave__8) * 0.5333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.0078125_f64), perm_table_minecraft_gravel_0_octave__7) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.007954116314199396_f64), perm_table_minecraft_gravel_1_octave__7) * 0.26666666666666666_f64));
        n = (n + (perlin((rpos3 * 0.015625_f64), perm_table_minecraft_gravel_0_octave__6) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.015908232628398793_f64), perm_table_minecraft_gravel_1_octave__6) * 0.13333333333333333_f64));
        n = (n + (perlin((rpos3 * 0.03125_f64), perm_table_minecraft_gravel_0_octave__5) * 0.06666666666666667_f64));
        n = (n + (perlin((rpos3 * 0.031816465256797585_f64), perm_table_minecraft_gravel_1_octave__5) * 0.06666666666666667_f64));
        return (n * 1.3333333333333333_f64);
    }
    
    pub fn minecraft_cave_layer_10_d5x1x5(out: &mut [f64; 25], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, perm_table_minecraft_cave_layer_0_octave__8: &PerlinNoiseSampler, perm_table_minecraft_cave_layer_1_octave__8: &PerlinNoiseSampler, host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut n: f64;
        for p in iter_3d(5_i32, 1_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * Vec3::new(0.15_f64, 0_f64, 0.15_f64)) + (pos3 * Vec3::new(0.6_f64, 0_f64, 0.6_f64)));
            n = 0_f64;
            n = (n + (perlin((rpos3 * 0.00390625_f64), perm_table_minecraft_cave_layer_0_octave__8) * 1_f64));
            n = (n + (perlin((rpos3 * 0.003977058157099698_f64), perm_table_minecraft_cave_layer_1_octave__8) * 1_f64));
            out[index(p)] = (n * 0.8333333333333333_f64);
        }
    }
    
    pub fn minecraft_cave_layer_73_d5x263x5(out: &mut [f64; 6575], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, perm_table_minecraft_cave_layer_0_octave__8: &PerlinNoiseSampler, perm_table_minecraft_cave_layer_1_octave__8: &PerlinNoiseSampler, host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut n: f64;
        for p in iter_3d(5_i32, 263_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * Vec3::new(5_f64, 2_f64, 5_f64)) + (pos3 * Vec3::new(20_f64, 16_f64, 20_f64)));
            n = 0_f64;
            n = (n + (perlin((rpos3 * 0.00390625_f64), perm_table_minecraft_cave_layer_0_octave__8) * 1_f64));
            n = (n + (perlin((rpos3 * 0.003977058157099698_f64), perm_table_minecraft_cave_layer_1_octave__8) * 1_f64));
            out[index(p)] = (n * 0.8333333333333333_f64);
        }
    }
    
    pub fn density_function_ShiftedNoise_27_d5x1x5(out: &mut [f64; 25], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, perm_table_minecraft_gravel_0_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__5: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__5: &PerlinNoiseSampler, host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        for p in iter_3d(5_i32, 1_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            out[index(p)] = density_function_ShiftedNoise_27_d5x1x5_minecraft_gravel_shifted_28(((rpos3 * Vec3::new(0.11_f64, 0_f64, 0.11_f64)) + Vec3::new(0_f64, 0_f64, 1_f64)), perm_table_minecraft_gravel_0_octave__8, perm_table_minecraft_gravel_1_octave__8, perm_table_minecraft_gravel_0_octave__7, perm_table_minecraft_gravel_1_octave__7, perm_table_minecraft_gravel_0_octave__6, perm_table_minecraft_gravel_1_octave__6, perm_table_minecraft_gravel_0_octave__5, perm_table_minecraft_gravel_1_octave__5, host_consts);
        }
    }
    
    pub fn minecraft_realism_hills_jagged_original_d5x263x5(out: &mut [f64; 6575], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207180296: &[f64; 25], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        for p in iter_3d(5_i32, 263_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            out[index(p)] = input_135471207180296[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize];
        }
    }
    
    pub fn minecraft_new_surface_combined_unprocessed_base_d5x263x5(out: &mut [f64; 6575], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207143088: &[f64; 6575], input_135471207142104: &[f64; 6575], input_135471207141232: &[f64; 6575], input_135471207131632: &[f64; 6575], input_135471207127872: &[f64; 6575], input_135471207121000: &[f64; 6575], input_135471207114128: &[f64; 6575], input_135471207112976: &[f64; 6575], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut var_0: f64;
        let mut spline_values_1: [f32; 1];
        let mut coordinate_2: f32;
        let mut spline_index_3: i32;
        let mut spline_values_4: [f32; 1];
        let mut coordinate_5: f32;
        let mut spline_index_6: i32;
        let mut spline_values_7: [f32; 2];
        let mut coordinate_8: f32;
        let mut spline_index_9: i32;
        let mut spline_values_10: [f32; 1];
        let mut coordinate_11: f32;
        let mut spline_index_12: i32;
        let mut spline_values_13: [f32; 3];
        let mut coordinate_14: f32;
        let mut spline_index_15: i32;
        let mut spline_values_16: [f32; 1];
        let mut coordinate_17: f32;
        let mut spline_index_18: i32;
        let mut spline_values_19: [f32; 4];
        let mut coordinate_20: f32;
        let mut spline_index_21: i32;
        let mut spline_values_22: [f32; 1];
        let mut coordinate_23: f32;
        let mut spline_index_24: i32;
        let mut spline_values_25: [f32; 5];
        let mut coordinate_26: f32;
        let mut spline_index_27: i32;
        let mut spline_values_28: [f32; 1];
        let mut coordinate_29: f32;
        let mut spline_index_30: i32;
        let mut spline_values_31: [f32; 6];
        let mut coordinate_32: f32;
        let mut spline_index_33: i32;
        let mut spline_values_34: [f32; 6];
        let mut coordinate_35: f32;
        let mut spline_index_36: i32;
        let mut spline_result_f32_37: f32;
        let mut spline_result_f64_38: f64;
        for p in iter_3d(5_i32, 263_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            var_0 = clamp(((rpos3.y - -64_f64) / 2096_f64), 0_f64, 1_f64);
            spline_values_1 = [0_f32];
            coordinate_2 = (input_135471207143088[index(p) as usize] as f32);
            spline_index_3 = binary_search(minecraft_new_surface_combined_unprocessed_base_spline_coordinates_135471207143960, (coordinate_2 as f32));
            spline_values_4 = [0_f32];
            coordinate_5 = (input_135471207142104[index(p) as usize] as f32);
            spline_index_6 = binary_search(minecraft_new_surface_combined_unprocessed_base_spline_coordinates_135471207142976, (coordinate_5 as f32));
            spline_values_7 = [advanced_hermite(minecraft_new_surface_combined_unprocessed_base_spline_coordinates_135471207143960, spline_values_1, minecraft_new_surface_combined_unprocessed_base_spline_derivatives_135471207143904, (coordinate_2 as f32), (spline_index_3 as i32)), advanced_hermite(minecraft_new_surface_combined_unprocessed_base_spline_coordinates_135471207142976, spline_values_4, minecraft_new_surface_combined_unprocessed_base_spline_derivatives_135471207142920, (coordinate_5 as f32), (spline_index_6 as i32))];
            coordinate_8 = (input_135471207141232[index(p) as usize] as f32);
            spline_index_9 = binary_search(minecraft_new_surface_combined_unprocessed_base_spline_coordinates_135471207141992, (coordinate_8 as f32));
            spline_values_10 = [0_f32];
            coordinate_11 = (input_135471207131632[index(p) as usize] as f32);
            spline_index_12 = binary_search(minecraft_new_surface_combined_unprocessed_base_spline_coordinates_135471207141120, (coordinate_11 as f32));
            spline_values_13 = [advanced_hermite(minecraft_new_surface_combined_unprocessed_base_spline_coordinates_135471207141120, spline_values_10, minecraft_new_surface_combined_unprocessed_base_spline_derivatives_135471207141064, (coordinate_11 as f32), (spline_index_12 as i32)), advanced_hermite(minecraft_new_surface_combined_unprocessed_base_spline_coordinates_135471207143960, spline_values_1, minecraft_new_surface_combined_unprocessed_base_spline_derivatives_135471207143904, (coordinate_2 as f32), (spline_index_3 as i32)), advanced_hermite(minecraft_new_surface_combined_unprocessed_base_spline_coordinates_135471207142976, spline_values_4, minecraft_new_surface_combined_unprocessed_base_spline_derivatives_135471207142920, (coordinate_5 as f32), (spline_index_6 as i32))];
            coordinate_14 = (input_135471207141232[index(p) as usize] as f32);
            spline_index_15 = binary_search(minecraft_new_surface_combined_unprocessed_base_spline_coordinates_135471207131520, (coordinate_14 as f32));
            spline_values_16 = [0_f32];
            coordinate_17 = (input_135471207127872[index(p) as usize] as f32);
            spline_index_18 = binary_search(minecraft_new_surface_combined_unprocessed_base_spline_coordinates_135471207131240, (coordinate_17 as f32));
            spline_values_19 = [advanced_hermite(minecraft_new_surface_combined_unprocessed_base_spline_coordinates_135471207131240, spline_values_16, minecraft_new_surface_combined_unprocessed_base_spline_derivatives_135471207131184, (coordinate_17 as f32), (spline_index_18 as i32)), advanced_hermite(minecraft_new_surface_combined_unprocessed_base_spline_coordinates_135471207141120, spline_values_10, minecraft_new_surface_combined_unprocessed_base_spline_derivatives_135471207141064, (coordinate_11 as f32), (spline_index_12 as i32)), advanced_hermite(minecraft_new_surface_combined_unprocessed_base_spline_coordinates_135471207143960, spline_values_1, minecraft_new_surface_combined_unprocessed_base_spline_derivatives_135471207143904, (coordinate_2 as f32), (spline_index_3 as i32)), advanced_hermite(minecraft_new_surface_combined_unprocessed_base_spline_coordinates_135471207142976, spline_values_4, minecraft_new_surface_combined_unprocessed_base_spline_derivatives_135471207142920, (coordinate_5 as f32), (spline_index_6 as i32))];
            coordinate_20 = (input_135471207141232[index(p) as usize] as f32);
            spline_index_21 = binary_search(minecraft_new_surface_combined_unprocessed_base_spline_coordinates_135471207127760, (coordinate_20 as f32));
            spline_values_22 = [0_f32];
            coordinate_23 = (input_135471207121000[index(p) as usize] as f32);
            spline_index_24 = binary_search(minecraft_new_surface_combined_unprocessed_base_spline_coordinates_135471207127480, (coordinate_23 as f32));
            spline_values_25 = [advanced_hermite(minecraft_new_surface_combined_unprocessed_base_spline_coordinates_135471207127480, spline_values_22, minecraft_new_surface_combined_unprocessed_base_spline_derivatives_135471207127424, (coordinate_23 as f32), (spline_index_24 as i32)), advanced_hermite(minecraft_new_surface_combined_unprocessed_base_spline_coordinates_135471207131240, spline_values_16, minecraft_new_surface_combined_unprocessed_base_spline_derivatives_135471207131184, (coordinate_17 as f32), (spline_index_18 as i32)), advanced_hermite(minecraft_new_surface_combined_unprocessed_base_spline_coordinates_135471207141120, spline_values_10, minecraft_new_surface_combined_unprocessed_base_spline_derivatives_135471207141064, (coordinate_11 as f32), (spline_index_12 as i32)), advanced_hermite(minecraft_new_surface_combined_unprocessed_base_spline_coordinates_135471207143960, spline_values_1, minecraft_new_surface_combined_unprocessed_base_spline_derivatives_135471207143904, (coordinate_2 as f32), (spline_index_3 as i32)), advanced_hermite(minecraft_new_surface_combined_unprocessed_base_spline_coordinates_135471207142976, spline_values_4, minecraft_new_surface_combined_unprocessed_base_spline_derivatives_135471207142920, (coordinate_5 as f32), (spline_index_6 as i32))];
            coordinate_26 = (input_135471207141232[index(p) as usize] as f32);
            spline_index_27 = binary_search(minecraft_new_surface_combined_unprocessed_base_spline_coordinates_135471207120888, (coordinate_26 as f32));
            spline_values_28 = [0_f32];
            coordinate_29 = (input_135471207114128[index(p) as usize] as f32);
            spline_index_30 = binary_search(minecraft_new_surface_combined_unprocessed_base_spline_coordinates_135471207120608, (coordinate_29 as f32));
            spline_values_31 = [advanced_hermite(minecraft_new_surface_combined_unprocessed_base_spline_coordinates_135471207120608, spline_values_28, minecraft_new_surface_combined_unprocessed_base_spline_derivatives_135471207120552, (coordinate_29 as f32), (spline_index_30 as i32)), advanced_hermite(minecraft_new_surface_combined_unprocessed_base_spline_coordinates_135471207127480, spline_values_22, minecraft_new_surface_combined_unprocessed_base_spline_derivatives_135471207127424, (coordinate_23 as f32), (spline_index_24 as i32)), advanced_hermite(minecraft_new_surface_combined_unprocessed_base_spline_coordinates_135471207131240, spline_values_16, minecraft_new_surface_combined_unprocessed_base_spline_derivatives_135471207131184, (coordinate_17 as f32), (spline_index_18 as i32)), advanced_hermite(minecraft_new_surface_combined_unprocessed_base_spline_coordinates_135471207141120, spline_values_10, minecraft_new_surface_combined_unprocessed_base_spline_derivatives_135471207141064, (coordinate_11 as f32), (spline_index_12 as i32)), advanced_hermite(minecraft_new_surface_combined_unprocessed_base_spline_coordinates_135471207143960, spline_values_1, minecraft_new_surface_combined_unprocessed_base_spline_derivatives_135471207143904, (coordinate_2 as f32), (spline_index_3 as i32)), advanced_hermite(minecraft_new_surface_combined_unprocessed_base_spline_coordinates_135471207142976, spline_values_4, minecraft_new_surface_combined_unprocessed_base_spline_derivatives_135471207142920, (coordinate_5 as f32), (spline_index_6 as i32))];
            coordinate_32 = (input_135471207141232[index(p) as usize] as f32);
            spline_index_33 = binary_search(minecraft_new_surface_combined_unprocessed_base_spline_coordinates_135471207114016, (coordinate_32 as f32));
            spline_values_34 = [advanced_hermite(minecraft_new_surface_combined_unprocessed_base_spline_coordinates_135471207141992, spline_values_7, minecraft_new_surface_combined_unprocessed_base_spline_derivatives_135471207141936, (coordinate_8 as f32), (spline_index_9 as i32)), advanced_hermite(minecraft_new_surface_combined_unprocessed_base_spline_coordinates_135471207131520, spline_values_13, minecraft_new_surface_combined_unprocessed_base_spline_derivatives_135471207131464, (coordinate_14 as f32), (spline_index_15 as i32)), advanced_hermite(minecraft_new_surface_combined_unprocessed_base_spline_coordinates_135471207131520, spline_values_13, minecraft_new_surface_combined_unprocessed_base_spline_derivatives_135471207131464, (coordinate_14 as f32), (spline_index_15 as i32)), advanced_hermite(minecraft_new_surface_combined_unprocessed_base_spline_coordinates_135471207127760, spline_values_19, minecraft_new_surface_combined_unprocessed_base_spline_derivatives_135471207127704, (coordinate_20 as f32), (spline_index_21 as i32)), advanced_hermite(minecraft_new_surface_combined_unprocessed_base_spline_coordinates_135471207120888, spline_values_25, minecraft_new_surface_combined_unprocessed_base_spline_derivatives_135471207120832, (coordinate_26 as f32), (spline_index_27 as i32)), advanced_hermite(minecraft_new_surface_combined_unprocessed_base_spline_coordinates_135471207114016, spline_values_31, minecraft_new_surface_combined_unprocessed_base_spline_derivatives_135471207113960, (coordinate_32 as f32), (spline_index_33 as i32))];
            coordinate_35 = (input_135471207112976[index(p) as usize] as f32);
            spline_index_36 = binary_search(minecraft_new_surface_combined_unprocessed_base_spline_coordinates_135471207113736, (coordinate_35 as f32));
            spline_result_f32_37 = advanced_hermite(minecraft_new_surface_combined_unprocessed_base_spline_coordinates_135471207113736, spline_values_34, minecraft_new_surface_combined_unprocessed_base_spline_derivatives_135471207113680, (coordinate_35 as f32), (spline_index_36 as i32));
            spline_result_f64_38 = (spline_result_f32_37 as f64);
            out[index(p)] = (((var_0 * -2_f64) + 1_f64) + spline_result_f64_38);
        }
    }
    
    pub fn minecraft_new_combination_unfixed_2_d5x263x5(out: &mut [f64; 6575], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207145616: &[f64; 6575], input_135471207144688: &[f64; 6575], input_135471207112608: &[f64; 6575], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut spline_values_0: [f32; 1];
        let mut coordinate_1: f32;
        let mut spline_index_2: i32;
        let mut spline_values_3: [f32; 1];
        let mut coordinate_4: f32;
        let mut spline_index_5: i32;
        let mut spline_values_6: [f32; 2];
        let mut coordinate_7: f32;
        let mut spline_index_8: i32;
        let mut spline_result_f32_9: f32;
        let mut spline_result_f64_10: f64;
        for p in iter_3d(5_i32, 263_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            spline_values_0 = [0_f32];
            coordinate_1 = (input_135471207145616[index(p) as usize] as f32);
            spline_index_2 = binary_search(minecraft_new_combination_unfixed_2_spline_coordinates_135471207111568, (coordinate_1 as f32));
            spline_values_3 = [0_f32];
            coordinate_4 = (input_135471207144688[index(p) as usize] as f32);
            spline_index_5 = binary_search(minecraft_new_combination_unfixed_2_spline_coordinates_135471207111232, (coordinate_4 as f32));
            spline_values_6 = [advanced_hermite(minecraft_new_combination_unfixed_2_spline_coordinates_135471207111568, spline_values_0, minecraft_new_combination_unfixed_2_spline_derivatives_135471207111512, (coordinate_1 as f32), (spline_index_2 as i32)), advanced_hermite(minecraft_new_combination_unfixed_2_spline_coordinates_135471207111232, spline_values_3, minecraft_new_combination_unfixed_2_spline_derivatives_135471207111176, (coordinate_4 as f32), (spline_index_5 as i32))];
            coordinate_7 = (input_135471207112608[index(p) as usize] as f32);
            spline_index_8 = binary_search(minecraft_new_combination_unfixed_2_spline_coordinates_135471207110896, (coordinate_7 as f32));
            spline_result_f32_9 = advanced_hermite(minecraft_new_combination_unfixed_2_spline_coordinates_135471207110896, spline_values_6, minecraft_new_combination_unfixed_2_spline_derivatives_135471207110840, (coordinate_7 as f32), (spline_index_8 as i32));
            spline_result_f64_10 = (spline_result_f32_9 as f64);
            out[index(p)] = spline_result_f64_10;
        }
    }
    
    pub fn density_function_Abs_65_d5x263x5(out: &mut [f64; 6575], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207156304: &[f64; 25], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        for p in iter_3d(5_i32, 263_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            out[index(p)] = abs(input_135471207156304[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize]);
        }
    }
    
    pub fn minecraft_caves_underlands_combined_d5x263x5(out: &mut [f64; 6575], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207155680: &[f64; 25], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut var_0: f64;
        for p in iter_3d(5_i32, 263_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            var_0 = clamp(((rpos3.y - -256_f64) / 1272_f64), 0_f64, 1_f64);
            out[index(p)] = (((var_0 * -2_f64) + 1_f64) + (input_135471207155680[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * -1.6_f64));
        }
    }
    
    pub fn density_function_ShiftedNoise_35_d5x1x5(out: &mut [f64; 25], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, perm_table_minecraft_gravel_0_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__5: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__5: &PerlinNoiseSampler, host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        for p in iter_3d(5_i32, 1_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            out[index(p)] = density_function_ShiftedNoise_35_d5x1x5_minecraft_gravel_shifted_36(((rpos3 * Vec3::new(0.12_f64, 0_f64, 0.12_f64)) + Vec3::new(0_f64, 0_f64, 1_f64)), perm_table_minecraft_gravel_0_octave__8, perm_table_minecraft_gravel_1_octave__8, perm_table_minecraft_gravel_0_octave__7, perm_table_minecraft_gravel_1_octave__7, perm_table_minecraft_gravel_0_octave__6, perm_table_minecraft_gravel_1_octave__6, perm_table_minecraft_gravel_0_octave__5, perm_table_minecraft_gravel_1_octave__5, host_consts);
        }
    }
    
    pub fn minecraft_realism_extreme_hills_slope_height_d5x1x5(out: &mut [f64; 25], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207194440: &[f64; 25], input_135471207187056: &[f64; 25], input_135471207181728: &[f64; 25], input_135471207174840: &[f64; 25], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut spline_values_0: [f32; 1];
        let mut coordinate_1: f32;
        let mut spline_index_2: i32;
        let mut spline_values_3: [f32; 1];
        let mut coordinate_4: f32;
        let mut spline_index_5: i32;
        let mut spline_values_6: [f32; 2];
        let mut coordinate_7: f32;
        let mut spline_index_8: i32;
        let mut spline_result_f32_9: f32;
        let mut spline_result_f64_10: f64;
        let mut spline_values_11: [f32; 1];
        let mut coordinate_12: f32;
        let mut spline_index_13: i32;
        let mut spline_values_14: [f32; 1];
        let mut coordinate_15: f32;
        let mut spline_index_16: i32;
        let mut spline_values_17: [f32; 2];
        let mut coordinate_18: f32;
        let mut spline_index_19: i32;
        let mut spline_result_f32_20: f32;
        let mut spline_result_f64_21: f64;
        for p in iter_3d(5_i32, 1_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            spline_values_0 = [0_f32];
            coordinate_1 = (input_135471207187056[index(p) as usize] as f32);
            spline_index_2 = binary_search(minecraft_realism_extreme_hills_slope_height_spline_coordinates_135471207177088, (coordinate_1 as f32));
            spline_values_3 = [0_f32];
            coordinate_4 = (input_135471207181728[index(p) as usize] as f32);
            spline_index_5 = binary_search(minecraft_realism_extreme_hills_slope_height_spline_coordinates_135471207176752, (coordinate_4 as f32));
            spline_values_6 = [advanced_hermite(minecraft_realism_extreme_hills_slope_height_spline_coordinates_135471207177088, spline_values_0, minecraft_realism_extreme_hills_slope_height_spline_derivatives_135471207177032, (coordinate_1 as f32), (spline_index_2 as i32)), advanced_hermite(minecraft_realism_extreme_hills_slope_height_spline_coordinates_135471207176752, spline_values_3, minecraft_realism_extreme_hills_slope_height_spline_derivatives_135471207176696, (coordinate_4 as f32), (spline_index_5 as i32))];
            coordinate_7 = (input_135471207187056[index(p) as usize] as f32);
            spline_index_8 = binary_search(minecraft_realism_extreme_hills_slope_height_spline_coordinates_135471207176416, (coordinate_7 as f32));
            spline_result_f32_9 = advanced_hermite(minecraft_realism_extreme_hills_slope_height_spline_coordinates_135471207176416, spline_values_6, minecraft_realism_extreme_hills_slope_height_spline_derivatives_135471207176360, (coordinate_7 as f32), (spline_index_8 as i32));
            spline_result_f64_10 = (spline_result_f32_9 as f64);
            spline_values_11 = [0_f32];
            coordinate_12 = (input_135471207174840[index(p) as usize] as f32);
            spline_index_13 = binary_search(minecraft_realism_extreme_hills_slope_height_spline_coordinates_135471207176024, (coordinate_12 as f32));
            spline_values_14 = [0_f32];
            coordinate_15 = (input_135471207174840[index(p) as usize] as f32);
            spline_index_16 = binary_search(minecraft_realism_extreme_hills_slope_height_spline_coordinates_135471207174728, (coordinate_15 as f32));
            spline_values_17 = [advanced_hermite(minecraft_realism_extreme_hills_slope_height_spline_coordinates_135471207176024, spline_values_11, minecraft_realism_extreme_hills_slope_height_spline_derivatives_135471207175968, (coordinate_12 as f32), (spline_index_13 as i32)), advanced_hermite(minecraft_realism_extreme_hills_slope_height_spline_coordinates_135471207174728, spline_values_14, minecraft_realism_extreme_hills_slope_height_spline_derivatives_135471207174672, (coordinate_15 as f32), (spline_index_16 as i32))];
            coordinate_18 = (input_135471207187056[index(p) as usize] as f32);
            spline_index_19 = binary_search(minecraft_realism_extreme_hills_slope_height_spline_coordinates_135471207174448, (coordinate_18 as f32));
            spline_result_f32_20 = advanced_hermite(minecraft_realism_extreme_hills_slope_height_spline_coordinates_135471207174448, spline_values_17, minecraft_realism_extreme_hills_slope_height_spline_derivatives_135471207174392, (coordinate_18 as f32), (spline_index_19 as i32));
            spline_result_f64_21 = (spline_result_f32_20 as f64);
            out[index(p)] = ((0_f64 + ((input_135471207194440[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * 0.4_f64) + -0.2_f64)) + ((spline_result_f64_10 * 0.3_f64) + spline_result_f64_21));
        }
    }
    
    pub fn density_function_Multiply_55_d5x263x5(out: &mut [f64; 6575], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207125032: &[f64; 6575], input_135471207172112: &[f64; 25], input_135471207123704: &[f64; 6575], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut var_0: f64;
        let mut var_1: f64;
        for p in iter_3d(5_i32, 263_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            var_0 = (input_135471207125032[index(p) as usize] + (input_135471207172112[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * -1_f64));
            var_1 = (input_135471207123704[index(p) as usize] + (input_135471207172112[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * -1_f64));
            out[index(p)] = ((((var_0 * var_0) + (var_1 * var_1)) * 5000_f64) * 0.2_f64);
        }
    }
    
    pub fn minecraft_realism_extreme_hills_factor_original_d5x263x5(out: &mut [f64; 6575], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207187312: &[f64; 25], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        for p in iter_3d(5_i32, 263_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            out[index(p)] = input_135471207187312[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize];
        }
    }
    
    pub fn density_function_ShiftedNoise_61_d5x263x5(out: &mut [f64; 6575], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, perm_table_minecraft_gravel_0_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__5: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__5: &PerlinNoiseSampler, host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        for p in iter_3d(5_i32, 263_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            out[index(p)] = density_function_ShiftedNoise_61_d5x263x5_minecraft_gravel_shifted_62(((rpos3 * Vec3::new(0.12_f64, 0_f64, 0.12_f64)) + Vec3::new(1_f64, 0_f64, 0_f64)), perm_table_minecraft_gravel_0_octave__8, perm_table_minecraft_gravel_1_octave__8, perm_table_minecraft_gravel_0_octave__7, perm_table_minecraft_gravel_1_octave__7, perm_table_minecraft_gravel_0_octave__6, perm_table_minecraft_gravel_1_octave__6, perm_table_minecraft_gravel_0_octave__5, perm_table_minecraft_gravel_1_octave__5, host_consts);
        }
    }
    
    pub fn density_function_ShiftedNoise_52_d5x263x5(out: &mut [f64; 6575], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, perm_table_minecraft_gravel_0_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__5: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__5: &PerlinNoiseSampler, host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        for p in iter_3d(5_i32, 263_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            out[index(p)] = density_function_ShiftedNoise_52_d5x263x5_minecraft_gravel_shifted_53(((rpos3 * Vec3::new(0.25_f64, 0_f64, 0.25_f64)) + Vec3::new(0_f64, 0_f64, 1_f64)), perm_table_minecraft_gravel_0_octave__8, perm_table_minecraft_gravel_1_octave__8, perm_table_minecraft_gravel_0_octave__7, perm_table_minecraft_gravel_1_octave__7, perm_table_minecraft_gravel_0_octave__6, perm_table_minecraft_gravel_1_octave__6, perm_table_minecraft_gravel_0_octave__5, perm_table_minecraft_gravel_1_octave__5, host_consts);
        }
    }
    
    pub fn minecraft_jagged_29_d5x1x5(out: &mut [f64; 25], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, perm_table_minecraft_jagged_0_octave__16: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__16: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__15: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__15: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__14: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__14: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__13: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__13: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__12: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__12: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__11: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__11: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__10: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__10: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__9: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__9: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__8: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__8: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__7: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__7: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__6: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__6: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__5: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__5: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__4: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__4: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__3: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__3: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__2: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__2: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__1: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__1: &PerlinNoiseSampler, host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut n: f64;
        for p in iter_3d(5_i32, 1_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * Vec3::new(501_f64, 0_f64, 501_f64)) + (pos3 * Vec3::new(2004_f64, 0_f64, 2004_f64)));
            n = 0_f64;
            n = (n + (perlin((rpos3 * 0.0000152587890625_f64), perm_table_minecraft_jagged_0_octave__16) * 0.5000076295109483_f64));
            n = (n + (perlin((rpos3 * 0.000015535383426170696_f64), perm_table_minecraft_jagged_1_octave__16) * 0.5000076295109483_f64));
            n = (n + (perlin((rpos3 * 0.000030517578125_f64), perm_table_minecraft_jagged_0_octave__15) * 0.2500038147554742_f64));
            n = (n + (perlin((rpos3 * 0.00003107076685234139_f64), perm_table_minecraft_jagged_1_octave__15) * 0.2500038147554742_f64));
            n = (n + (perlin((rpos3 * 0.00006103515625_f64), perm_table_minecraft_jagged_0_octave__14) * 0.1250019073777371_f64));
            n = (n + (perlin((rpos3 * 0.00006214153370468278_f64), perm_table_minecraft_jagged_1_octave__14) * 0.1250019073777371_f64));
            n = (n + (perlin((rpos3 * 0.0001220703125_f64), perm_table_minecraft_jagged_0_octave__13) * 0.06250095368886854_f64));
            n = (n + (perlin((rpos3 * 0.00012428306740936557_f64), perm_table_minecraft_jagged_1_octave__13) * 0.06250095368886854_f64));
            n = (n + (perlin((rpos3 * 0.000244140625_f64), perm_table_minecraft_jagged_0_octave__12) * 0.03125047684443427_f64));
            n = (n + (perlin((rpos3 * 0.00024856613481873114_f64), perm_table_minecraft_jagged_1_octave__12) * 0.03125047684443427_f64));
            n = (n + (perlin((rpos3 * 0.00048828125_f64), perm_table_minecraft_jagged_0_octave__11) * 0.015625238422217136_f64));
            n = (n + (perlin((rpos3 * 0.0004971322696374623_f64), perm_table_minecraft_jagged_1_octave__11) * 0.015625238422217136_f64));
            n = (n + (perlin((rpos3 * 0.0009765625_f64), perm_table_minecraft_jagged_0_octave__10) * 0.007812619211108568_f64));
            n = (n + (perlin((rpos3 * 0.0009942645392749245_f64), perm_table_minecraft_jagged_1_octave__10) * 0.007812619211108568_f64));
            n = (n + (perlin((rpos3 * 0.001953125_f64), perm_table_minecraft_jagged_0_octave__9) * 0.003906309605554284_f64));
            n = (n + (perlin((rpos3 * 0.001988529078549849_f64), perm_table_minecraft_jagged_1_octave__9) * 0.003906309605554284_f64));
            n = (n + (perlin((rpos3 * 0.00390625_f64), perm_table_minecraft_jagged_0_octave__8) * 0.001953154802777142_f64));
            n = (n + (perlin((rpos3 * 0.003977058157099698_f64), perm_table_minecraft_jagged_1_octave__8) * 0.001953154802777142_f64));
            n = (n + (perlin((rpos3 * 0.0078125_f64), perm_table_minecraft_jagged_0_octave__7) * 0.000976577401388571_f64));
            n = (n + (perlin((rpos3 * 0.007954116314199396_f64), perm_table_minecraft_jagged_1_octave__7) * 0.000976577401388571_f64));
            n = (n + (perlin((rpos3 * 0.015625_f64), perm_table_minecraft_jagged_0_octave__6) * 0.0004882887006942855_f64));
            n = (n + (perlin((rpos3 * 0.015908232628398793_f64), perm_table_minecraft_jagged_1_octave__6) * 0.0004882887006942855_f64));
            n = (n + (perlin((rpos3 * 0.03125_f64), perm_table_minecraft_jagged_0_octave__5) * 0.00024414435034714275_f64));
            n = (n + (perlin((rpos3 * 0.031816465256797585_f64), perm_table_minecraft_jagged_1_octave__5) * 0.00024414435034714275_f64));
            n = (n + (perlin((rpos3 * 0.0625_f64), perm_table_minecraft_jagged_0_octave__4) * 0.00012207217517357137_f64));
            n = (n + (perlin((rpos3 * 0.06363293051359517_f64), perm_table_minecraft_jagged_1_octave__4) * 0.00012207217517357137_f64));
            n = (n + (perlin((rpos3 * 0.125_f64), perm_table_minecraft_jagged_0_octave__3) * 0.00006103608758678569_f64));
            n = (n + (perlin((rpos3 * 0.12726586102719034_f64), perm_table_minecraft_jagged_1_octave__3) * 0.00006103608758678569_f64));
            n = (n + (perlin((rpos3 * 0.25_f64), perm_table_minecraft_jagged_0_octave__2) * 0.000030518043793392844_f64));
            n = (n + (perlin((rpos3 * 0.2545317220543807_f64), perm_table_minecraft_jagged_1_octave__2) * 0.000030518043793392844_f64));
            n = (n + (perlin((rpos3 * 0.5_f64), perm_table_minecraft_jagged_0_octave__1) * 0.000015259021896696422_f64));
            n = (n + (perlin((rpos3 * 0.5090634441087614_f64), perm_table_minecraft_jagged_1_octave__1) * 0.000015259021896696422_f64));
            out[index(p)] = (n * 1.568627450980392_f64);
        }
    }
    
    pub fn density_function_YClampedGradient_76_d5x263x5(out: &mut [f64; 6575], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut var_0: f64;
        for p in iter_3d(5_i32, 263_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            var_0 = clamp(((rpos3.y - -64_f64) / 2096_f64), 0_f64, 1_f64);
            out[index(p)] = ((var_0 * 2096_f64) + -64_f64);
        }
    }
    
    pub fn density_function_Multiply_12_d5x1x5(out: &mut [f64; 25], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207187312: &[f64; 25], input_135471207182096: &[f64; 25], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut spline_values_0: [f32; 17];
        let mut coordinate_1: f32;
        let mut spline_index_2: i32;
        let mut spline_result_f32_3: f32;
        let mut spline_result_f64_4: f64;
        for p in iter_3d(5_i32, 1_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            spline_values_0 = [1_f32, 0.95238096_f32, 0.90909094_f32, 0.8333333_f32, 0.7692308_f32, 0.71428573_f32, 0.6666667_f32, 0.625_f32, 0.5882353_f32, 0.5555556_f32, 0.5263158_f32, 0.5_f32, 0.47619048_f32, 0.45454547_f32, 0.4347826_f32, 0.41666666_f32, 0.4_f32];
            coordinate_1 = (input_135471207182096[index(p) as usize] as f32);
            spline_index_2 = binary_search(density_function_Multiply_12_spline_coordinates_135471207186328, (coordinate_1 as f32));
            spline_result_f32_3 = advanced_hermite(density_function_Multiply_12_spline_coordinates_135471207186328, spline_values_0, density_function_Multiply_12_spline_derivatives_135471207186272, (coordinate_1 as f32), (spline_index_2 as i32));
            spline_result_f64_4 = (spline_result_f32_3 as f64);
            out[index(p)] = (input_135471207187312[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * (spline_result_f64_4 * 0.3_f64));
        }
    }
    
    pub fn minecraft_determiner_cave_1_d5x263x5(out: &mut [f64; 6575], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207100376: &[f64; 25], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut var_0: f64;
        for p in iter_3d(5_i32, 263_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            var_0 = clamp(((rpos3.y - -64_f64) / 2096_f64), 0_f64, 1_f64);
            out[index(p)] = (((var_0 * -2_f64) + 1_f64) + input_135471207100376[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize]);
        }
    }
    
    pub fn minecraft_jagged_9_d5x1x5(out: &mut [f64; 25], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, perm_table_minecraft_jagged_0_octave__16: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__16: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__15: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__15: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__14: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__14: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__13: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__13: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__12: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__12: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__11: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__11: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__10: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__10: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__9: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__9: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__8: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__8: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__7: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__7: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__6: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__6: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__5: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__5: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__4: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__4: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__3: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__3: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__2: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__2: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__1: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__1: &PerlinNoiseSampler, host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut n: f64;
        for p in iter_3d(5_i32, 1_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * Vec3::new(500_f64, 0_f64, 500_f64)) + (pos3 * Vec3::new(2000_f64, 0_f64, 2000_f64)));
            n = 0_f64;
            n = (n + (perlin((rpos3 * 0.0000152587890625_f64), perm_table_minecraft_jagged_0_octave__16) * 0.5000076295109483_f64));
            n = (n + (perlin((rpos3 * 0.000015535383426170696_f64), perm_table_minecraft_jagged_1_octave__16) * 0.5000076295109483_f64));
            n = (n + (perlin((rpos3 * 0.000030517578125_f64), perm_table_minecraft_jagged_0_octave__15) * 0.2500038147554742_f64));
            n = (n + (perlin((rpos3 * 0.00003107076685234139_f64), perm_table_minecraft_jagged_1_octave__15) * 0.2500038147554742_f64));
            n = (n + (perlin((rpos3 * 0.00006103515625_f64), perm_table_minecraft_jagged_0_octave__14) * 0.1250019073777371_f64));
            n = (n + (perlin((rpos3 * 0.00006214153370468278_f64), perm_table_minecraft_jagged_1_octave__14) * 0.1250019073777371_f64));
            n = (n + (perlin((rpos3 * 0.0001220703125_f64), perm_table_minecraft_jagged_0_octave__13) * 0.06250095368886854_f64));
            n = (n + (perlin((rpos3 * 0.00012428306740936557_f64), perm_table_minecraft_jagged_1_octave__13) * 0.06250095368886854_f64));
            n = (n + (perlin((rpos3 * 0.000244140625_f64), perm_table_minecraft_jagged_0_octave__12) * 0.03125047684443427_f64));
            n = (n + (perlin((rpos3 * 0.00024856613481873114_f64), perm_table_minecraft_jagged_1_octave__12) * 0.03125047684443427_f64));
            n = (n + (perlin((rpos3 * 0.00048828125_f64), perm_table_minecraft_jagged_0_octave__11) * 0.015625238422217136_f64));
            n = (n + (perlin((rpos3 * 0.0004971322696374623_f64), perm_table_minecraft_jagged_1_octave__11) * 0.015625238422217136_f64));
            n = (n + (perlin((rpos3 * 0.0009765625_f64), perm_table_minecraft_jagged_0_octave__10) * 0.007812619211108568_f64));
            n = (n + (perlin((rpos3 * 0.0009942645392749245_f64), perm_table_minecraft_jagged_1_octave__10) * 0.007812619211108568_f64));
            n = (n + (perlin((rpos3 * 0.001953125_f64), perm_table_minecraft_jagged_0_octave__9) * 0.003906309605554284_f64));
            n = (n + (perlin((rpos3 * 0.001988529078549849_f64), perm_table_minecraft_jagged_1_octave__9) * 0.003906309605554284_f64));
            n = (n + (perlin((rpos3 * 0.00390625_f64), perm_table_minecraft_jagged_0_octave__8) * 0.001953154802777142_f64));
            n = (n + (perlin((rpos3 * 0.003977058157099698_f64), perm_table_minecraft_jagged_1_octave__8) * 0.001953154802777142_f64));
            n = (n + (perlin((rpos3 * 0.0078125_f64), perm_table_minecraft_jagged_0_octave__7) * 0.000976577401388571_f64));
            n = (n + (perlin((rpos3 * 0.007954116314199396_f64), perm_table_minecraft_jagged_1_octave__7) * 0.000976577401388571_f64));
            n = (n + (perlin((rpos3 * 0.015625_f64), perm_table_minecraft_jagged_0_octave__6) * 0.0004882887006942855_f64));
            n = (n + (perlin((rpos3 * 0.015908232628398793_f64), perm_table_minecraft_jagged_1_octave__6) * 0.0004882887006942855_f64));
            n = (n + (perlin((rpos3 * 0.03125_f64), perm_table_minecraft_jagged_0_octave__5) * 0.00024414435034714275_f64));
            n = (n + (perlin((rpos3 * 0.031816465256797585_f64), perm_table_minecraft_jagged_1_octave__5) * 0.00024414435034714275_f64));
            n = (n + (perlin((rpos3 * 0.0625_f64), perm_table_minecraft_jagged_0_octave__4) * 0.00012207217517357137_f64));
            n = (n + (perlin((rpos3 * 0.06363293051359517_f64), perm_table_minecraft_jagged_1_octave__4) * 0.00012207217517357137_f64));
            n = (n + (perlin((rpos3 * 0.125_f64), perm_table_minecraft_jagged_0_octave__3) * 0.00006103608758678569_f64));
            n = (n + (perlin((rpos3 * 0.12726586102719034_f64), perm_table_minecraft_jagged_1_octave__3) * 0.00006103608758678569_f64));
            n = (n + (perlin((rpos3 * 0.25_f64), perm_table_minecraft_jagged_0_octave__2) * 0.000030518043793392844_f64));
            n = (n + (perlin((rpos3 * 0.2545317220543807_f64), perm_table_minecraft_jagged_1_octave__2) * 0.000030518043793392844_f64));
            n = (n + (perlin((rpos3 * 0.5_f64), perm_table_minecraft_jagged_0_octave__1) * 0.000015259021896696422_f64));
            n = (n + (perlin((rpos3 * 0.5090634441087614_f64), perm_table_minecraft_jagged_1_octave__1) * 0.000015259021896696422_f64));
            out[index(p)] = (n * 1.568627450980392_f64);
        }
    }
    
    pub fn minecraft_cave_layer_30_d5x1x5(out: &mut [f64; 25], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, perm_table_minecraft_cave_layer_0_octave__8: &PerlinNoiseSampler, perm_table_minecraft_cave_layer_1_octave__8: &PerlinNoiseSampler, host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut n: f64;
        for p in iter_3d(5_i32, 1_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * Vec3::new(0.25_f64, 0_f64, 0.25_f64)) + (pos3 * Vec3::new(1_f64, 0_f64, 1_f64)));
            n = 0_f64;
            n = (n + (perlin((rpos3 * 0.00390625_f64), perm_table_minecraft_cave_layer_0_octave__8) * 1_f64));
            n = (n + (perlin((rpos3 * 0.003977058157099698_f64), perm_table_minecraft_cave_layer_1_octave__8) * 1_f64));
            out[index(p)] = (n * 0.8333333333333333_f64);
        }
    }
    
    pub fn minecraft_cave_layer_67_d5x263x5(out: &mut [f64; 6575], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, perm_table_minecraft_cave_layer_0_octave__8: &PerlinNoiseSampler, perm_table_minecraft_cave_layer_1_octave__8: &PerlinNoiseSampler, host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut n: f64;
        for p in iter_3d(5_i32, 263_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = (origin + (pos3 * Vec3::new(4_f64, 8_f64, 4_f64)));
            n = 0_f64;
            n = (n + (perlin((rpos3 * 0.00390625_f64), perm_table_minecraft_cave_layer_0_octave__8) * 1_f64));
            n = (n + (perlin((rpos3 * 0.003977058157099698_f64), perm_table_minecraft_cave_layer_1_octave__8) * 1_f64));
            out[index(p)] = (n * 0.8333333333333333_f64);
        }
    }
    
    pub fn minecraft_caves_old_small_d5x263x5(out: &mut [f64; 6575], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207108000: &[f64; 6575], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        for p in iter_3d(5_i32, 263_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            out[index(p)] = input_135471207108000[index(p) as usize];
        }
    }
    
    pub fn minecraft_new_surface_combined_processed_d5x263x5(out: &mut [f64; 6575], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207145816: &[f64; 25], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut var_0: f64;
        for p in iter_3d(5_i32, 263_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            var_0 = clamp(((rpos3.y - -64_f64) / 2096_f64), 0_f64, 1_f64);
            out[index(p)] = (((var_0 * -2_f64) + 1_f64) + input_135471207145816[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize]);
        }
    }
    
    pub fn minecraft_jagged_18_d5x1x5(out: &mut [f64; 25], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, perm_table_minecraft_jagged_0_octave__16: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__16: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__15: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__15: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__14: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__14: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__13: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__13: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__12: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__12: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__11: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__11: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__10: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__10: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__9: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__9: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__8: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__8: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__7: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__7: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__6: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__6: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__5: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__5: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__4: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__4: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__3: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__3: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__2: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__2: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__1: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__1: &PerlinNoiseSampler, host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut n: f64;
        for p in iter_3d(5_i32, 1_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * Vec3::new(250_f64, 0_f64, 250_f64)) + (pos3 * Vec3::new(1000_f64, 0_f64, 1000_f64)));
            n = 0_f64;
            n = (n + (perlin((rpos3 * 0.0000152587890625_f64), perm_table_minecraft_jagged_0_octave__16) * 0.5000076295109483_f64));
            n = (n + (perlin((rpos3 * 0.000015535383426170696_f64), perm_table_minecraft_jagged_1_octave__16) * 0.5000076295109483_f64));
            n = (n + (perlin((rpos3 * 0.000030517578125_f64), perm_table_minecraft_jagged_0_octave__15) * 0.2500038147554742_f64));
            n = (n + (perlin((rpos3 * 0.00003107076685234139_f64), perm_table_minecraft_jagged_1_octave__15) * 0.2500038147554742_f64));
            n = (n + (perlin((rpos3 * 0.00006103515625_f64), perm_table_minecraft_jagged_0_octave__14) * 0.1250019073777371_f64));
            n = (n + (perlin((rpos3 * 0.00006214153370468278_f64), perm_table_minecraft_jagged_1_octave__14) * 0.1250019073777371_f64));
            n = (n + (perlin((rpos3 * 0.0001220703125_f64), perm_table_minecraft_jagged_0_octave__13) * 0.06250095368886854_f64));
            n = (n + (perlin((rpos3 * 0.00012428306740936557_f64), perm_table_minecraft_jagged_1_octave__13) * 0.06250095368886854_f64));
            n = (n + (perlin((rpos3 * 0.000244140625_f64), perm_table_minecraft_jagged_0_octave__12) * 0.03125047684443427_f64));
            n = (n + (perlin((rpos3 * 0.00024856613481873114_f64), perm_table_minecraft_jagged_1_octave__12) * 0.03125047684443427_f64));
            n = (n + (perlin((rpos3 * 0.00048828125_f64), perm_table_minecraft_jagged_0_octave__11) * 0.015625238422217136_f64));
            n = (n + (perlin((rpos3 * 0.0004971322696374623_f64), perm_table_minecraft_jagged_1_octave__11) * 0.015625238422217136_f64));
            n = (n + (perlin((rpos3 * 0.0009765625_f64), perm_table_minecraft_jagged_0_octave__10) * 0.007812619211108568_f64));
            n = (n + (perlin((rpos3 * 0.0009942645392749245_f64), perm_table_minecraft_jagged_1_octave__10) * 0.007812619211108568_f64));
            n = (n + (perlin((rpos3 * 0.001953125_f64), perm_table_minecraft_jagged_0_octave__9) * 0.003906309605554284_f64));
            n = (n + (perlin((rpos3 * 0.001988529078549849_f64), perm_table_minecraft_jagged_1_octave__9) * 0.003906309605554284_f64));
            n = (n + (perlin((rpos3 * 0.00390625_f64), perm_table_minecraft_jagged_0_octave__8) * 0.001953154802777142_f64));
            n = (n + (perlin((rpos3 * 0.003977058157099698_f64), perm_table_minecraft_jagged_1_octave__8) * 0.001953154802777142_f64));
            n = (n + (perlin((rpos3 * 0.0078125_f64), perm_table_minecraft_jagged_0_octave__7) * 0.000976577401388571_f64));
            n = (n + (perlin((rpos3 * 0.007954116314199396_f64), perm_table_minecraft_jagged_1_octave__7) * 0.000976577401388571_f64));
            n = (n + (perlin((rpos3 * 0.015625_f64), perm_table_minecraft_jagged_0_octave__6) * 0.0004882887006942855_f64));
            n = (n + (perlin((rpos3 * 0.015908232628398793_f64), perm_table_minecraft_jagged_1_octave__6) * 0.0004882887006942855_f64));
            n = (n + (perlin((rpos3 * 0.03125_f64), perm_table_minecraft_jagged_0_octave__5) * 0.00024414435034714275_f64));
            n = (n + (perlin((rpos3 * 0.031816465256797585_f64), perm_table_minecraft_jagged_1_octave__5) * 0.00024414435034714275_f64));
            n = (n + (perlin((rpos3 * 0.0625_f64), perm_table_minecraft_jagged_0_octave__4) * 0.00012207217517357137_f64));
            n = (n + (perlin((rpos3 * 0.06363293051359517_f64), perm_table_minecraft_jagged_1_octave__4) * 0.00012207217517357137_f64));
            n = (n + (perlin((rpos3 * 0.125_f64), perm_table_minecraft_jagged_0_octave__3) * 0.00006103608758678569_f64));
            n = (n + (perlin((rpos3 * 0.12726586102719034_f64), perm_table_minecraft_jagged_1_octave__3) * 0.00006103608758678569_f64));
            n = (n + (perlin((rpos3 * 0.25_f64), perm_table_minecraft_jagged_0_octave__2) * 0.000030518043793392844_f64));
            n = (n + (perlin((rpos3 * 0.2545317220543807_f64), perm_table_minecraft_jagged_1_octave__2) * 0.000030518043793392844_f64));
            n = (n + (perlin((rpos3 * 0.5_f64), perm_table_minecraft_jagged_0_octave__1) * 0.000015259021896696422_f64));
            n = (n + (perlin((rpos3 * 0.5090634441087614_f64), perm_table_minecraft_jagged_1_octave__1) * 0.000015259021896696422_f64));
            out[index(p)] = (n * 1.568627450980392_f64);
        }
    }
    
    pub fn minecraft_realism_extreme_mountains_slope_height_d5x1x5(out: &mut [f64; 25], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207164448: &[f64; 25], input_135471207163968: &[f64; 25], input_135471207160552: &[f64; 25], input_135471207159144: &[f64; 25], input_135471207157992: &[f64; 25], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut spline_values_0: [f32; 17];
        let mut coordinate_1: f32;
        let mut spline_index_2: i32;
        let mut spline_result_f32_3: f32;
        let mut spline_result_f64_4: f64;
        let mut spline_values_5: [f32; 1];
        let mut coordinate_6: f32;
        let mut spline_index_7: i32;
        let mut spline_values_8: [f32; 1];
        let mut coordinate_9: f32;
        let mut spline_index_10: i32;
        let mut spline_values_11: [f32; 2];
        let mut coordinate_12: f32;
        let mut spline_index_13: i32;
        let mut spline_result_f32_14: f32;
        let mut spline_result_f64_15: f64;
        for p in iter_3d(5_i32, 1_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            spline_values_0 = [1_f32, 0.95238096_f32, 0.90909094_f32, 0.8333333_f32, 0.7692308_f32, 0.71428573_f32, 0.6666667_f32, 0.625_f32, 0.5882353_f32, 0.5555556_f32, 0.5263158_f32, 0.5_f32, 0.47619048_f32, 0.45454547_f32, 0.4347826_f32, 0.41666666_f32, 0.4_f32];
            coordinate_1 = (input_135471207160552[index(p) as usize] as f32);
            spline_index_2 = binary_search(minecraft_realism_extreme_mountains_slope_height_spline_coordinates_135471207163856, (coordinate_1 as f32));
            spline_result_f32_3 = advanced_hermite(minecraft_realism_extreme_mountains_slope_height_spline_coordinates_135471207163856, spline_values_0, minecraft_realism_extreme_mountains_slope_height_spline_derivatives_135471207163800, (coordinate_1 as f32), (spline_index_2 as i32));
            spline_result_f64_4 = (spline_result_f32_3 as f64);
            spline_values_5 = [0_f32];
            coordinate_6 = (input_135471207159144[index(p) as usize] as f32);
            spline_index_7 = binary_search(minecraft_realism_extreme_mountains_slope_height_spline_coordinates_135471207160328, (coordinate_6 as f32));
            spline_values_8 = [0_f32];
            coordinate_9 = (input_135471207159144[index(p) as usize] as f32);
            spline_index_10 = binary_search(minecraft_realism_extreme_mountains_slope_height_spline_coordinates_135471207159032, (coordinate_9 as f32));
            spline_values_11 = [advanced_hermite(minecraft_realism_extreme_mountains_slope_height_spline_coordinates_135471207160328, spline_values_5, minecraft_realism_extreme_mountains_slope_height_spline_derivatives_135471207160272, (coordinate_6 as f32), (spline_index_7 as i32)), advanced_hermite(minecraft_realism_extreme_mountains_slope_height_spline_coordinates_135471207159032, spline_values_8, minecraft_realism_extreme_mountains_slope_height_spline_derivatives_135471207158976, (coordinate_9 as f32), (spline_index_10 as i32))];
            coordinate_12 = (input_135471207157992[index(p) as usize] as f32);
            spline_index_13 = binary_search(minecraft_realism_extreme_mountains_slope_height_spline_coordinates_135471207158752, (coordinate_12 as f32));
            spline_result_f32_14 = advanced_hermite(minecraft_realism_extreme_mountains_slope_height_spline_coordinates_135471207158752, spline_values_11, minecraft_realism_extreme_mountains_slope_height_spline_derivatives_135471207158696, (coordinate_12 as f32), (spline_index_13 as i32));
            spline_result_f64_15 = (spline_result_f32_14 as f64);
            out[index(p)] = ((0_f64 + ((input_135471207164448[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * 0.2_f64) + 0.4_f64)) + (((input_135471207163968[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * spline_result_f64_4) * 0.9_f64) + spline_result_f64_15));
        }
    }
    
    pub fn density_function_Multiply_48_d5x263x5(out: &mut [f64; 6575], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207187312: &[f64; 25], input_135471207134784: &[f64; 6575], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut spline_values_0: [f32; 17];
        let mut coordinate_1: f32;
        let mut spline_index_2: i32;
        let mut spline_result_f32_3: f32;
        let mut spline_result_f64_4: f64;
        for p in iter_3d(5_i32, 263_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            spline_values_0 = [1_f32, 0.95238096_f32, 0.90909094_f32, 0.8333333_f32, 0.7692308_f32, 0.71428573_f32, 0.6666667_f32, 0.625_f32, 0.5882353_f32, 0.5555556_f32, 0.5263158_f32, 0.5_f32, 0.47619048_f32, 0.45454547_f32, 0.4347826_f32, 0.41666666_f32, 0.4_f32];
            coordinate_1 = (input_135471207134784[index(p) as usize] as f32);
            spline_index_2 = binary_search(density_function_Multiply_48_spline_coordinates_135471207139016, (coordinate_1 as f32));
            spline_result_f32_3 = advanced_hermite(density_function_Multiply_48_spline_coordinates_135471207139016, spline_values_0, density_function_Multiply_48_spline_derivatives_135471207138960, (coordinate_1 as f32), (spline_index_2 as i32));
            spline_result_f64_4 = (spline_result_f32_3 as f64);
            out[index(p)] = (input_135471207187312[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * (spline_result_f64_4 * 0.3_f64));
        }
    }
    
    pub fn minecraft_caves_new_small_d5x263x5(out: &mut [f64; 6575], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207100120: &[f64; 6575], input_135471207099528: &[f64; 6575], input_135471207099048: &[f64; 25], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut spline_values_0: [f32; 165];
        let mut coordinate_1: f32;
        let mut spline_index_2: i32;
        let mut spline_result_f32_3: f32;
        let mut spline_result_f64_4: f64;
        for p in iter_3d(5_i32, 263_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            spline_values_0 = [1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32];
            coordinate_1 = (input_135471207100120[index(p) as usize] as f32);
            spline_index_2 = binary_search(minecraft_caves_new_small_spline_coordinates_135471207101360, (coordinate_1 as f32));
            spline_result_f32_3 = advanced_hermite(minecraft_caves_new_small_spline_coordinates_135471207101360, spline_values_0, minecraft_caves_new_small_spline_derivatives_135471207101304, (coordinate_1 as f32), (spline_index_2 as i32));
            spline_result_f64_4 = (spline_result_f32_3 as f64);
            out[index(p)] = (spline_result_f64_4 + ((input_135471207099528[index(p) as usize] * 1.5_f64) + (input_135471207099048[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * 0.5_f64)));
        }
    }
    
    pub fn final_density_d16x2096x16(out: &mut [f64; 536576], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207090832: &[f64; 6575], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        for p in iter_3d(16_i32, 2096_i32, 16_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            out[index(p)] = interpolate(input_135471207090832[cornerx0y0z0_8(pos3, 5_i32, 263_i32, 5_i32) as usize], input_135471207090832[cornerx4y0z0_8(pos3, 5_i32, 263_i32, 5_i32) as usize], input_135471207090832[cornerx0y8z0_8(pos3, 5_i32, 263_i32, 5_i32) as usize], input_135471207090832[cornerx4y8z0_8(pos3, 5_i32, 263_i32, 5_i32) as usize], input_135471207090832[cornerx0y0z4_8(pos3, 5_i32, 263_i32, 5_i32) as usize], input_135471207090832[cornerx4y0z4_8(pos3, 5_i32, 263_i32, 5_i32) as usize], input_135471207090832[cornerx0y8z4_8(pos3, 5_i32, 263_i32, 5_i32) as usize], input_135471207090832[cornerx4y8z4_8(pos3, 5_i32, 263_i32, 5_i32) as usize], xfract4(pos3), yfract8(pos3), zfract4(pos3));
        }
    }
    
    pub fn minecraft_gravel_11_d5x1x5(out: &mut [f64; 25], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, perm_table_minecraft_gravel_0_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__5: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__5: &PerlinNoiseSampler, host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut n: f64;
        for p in iter_3d(5_i32, 1_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * Vec3::new(0.25_f64, 0_f64, 0.25_f64)) + (pos3 * Vec3::new(1_f64, 0_f64, 1_f64)));
            n = 0_f64;
            n = (n + (perlin((rpos3 * 0.00390625_f64), perm_table_minecraft_gravel_0_octave__8) * 0.5333333333333333_f64));
            n = (n + (perlin((rpos3 * 0.003977058157099698_f64), perm_table_minecraft_gravel_1_octave__8) * 0.5333333333333333_f64));
            n = (n + (perlin((rpos3 * 0.0078125_f64), perm_table_minecraft_gravel_0_octave__7) * 0.26666666666666666_f64));
            n = (n + (perlin((rpos3 * 0.007954116314199396_f64), perm_table_minecraft_gravel_1_octave__7) * 0.26666666666666666_f64));
            n = (n + (perlin((rpos3 * 0.015625_f64), perm_table_minecraft_gravel_0_octave__6) * 0.13333333333333333_f64));
            n = (n + (perlin((rpos3 * 0.015908232628398793_f64), perm_table_minecraft_gravel_1_octave__6) * 0.13333333333333333_f64));
            n = (n + (perlin((rpos3 * 0.03125_f64), perm_table_minecraft_gravel_0_octave__5) * 0.06666666666666667_f64));
            n = (n + (perlin((rpos3 * 0.031816465256797585_f64), perm_table_minecraft_gravel_1_octave__5) * 0.06666666666666667_f64));
            out[index(p)] = (n * 1.3333333333333333_f64);
        }
    }
    
    pub fn minecraft_realism_extreme_mountains_jagged_original_d5x263x5(out: &mut [f64; 6575], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207159400: &[f64; 25], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        for p in iter_3d(5_i32, 263_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            out[index(p)] = input_135471207159400[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize];
        }
    }
    
    pub fn minecraft_caves_barrier_1_d5x263x5(out: &mut [f64; 6575], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207093032: &[f64; 6575], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        for p in iter_3d(5_i32, 263_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            out[index(p)] = input_135471207093032[index(p) as usize];
        }
    }
    
    pub fn minecraft_gravel_70_d5x1x5(out: &mut [f64; 25], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, perm_table_minecraft_gravel_0_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__5: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__5: &PerlinNoiseSampler, host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut n: f64;
        for p in iter_3d(5_i32, 1_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * Vec3::new(0.03_f64, 0_f64, 0.03_f64)) + (pos3 * Vec3::new(0.12_f64, 0_f64, 0.12_f64)));
            n = 0_f64;
            n = (n + (perlin((rpos3 * 0.00390625_f64), perm_table_minecraft_gravel_0_octave__8) * 0.5333333333333333_f64));
            n = (n + (perlin((rpos3 * 0.003977058157099698_f64), perm_table_minecraft_gravel_1_octave__8) * 0.5333333333333333_f64));
            n = (n + (perlin((rpos3 * 0.0078125_f64), perm_table_minecraft_gravel_0_octave__7) * 0.26666666666666666_f64));
            n = (n + (perlin((rpos3 * 0.007954116314199396_f64), perm_table_minecraft_gravel_1_octave__7) * 0.26666666666666666_f64));
            n = (n + (perlin((rpos3 * 0.015625_f64), perm_table_minecraft_gravel_0_octave__6) * 0.13333333333333333_f64));
            n = (n + (perlin((rpos3 * 0.015908232628398793_f64), perm_table_minecraft_gravel_1_octave__6) * 0.13333333333333333_f64));
            n = (n + (perlin((rpos3 * 0.03125_f64), perm_table_minecraft_gravel_0_octave__5) * 0.06666666666666667_f64));
            n = (n + (perlin((rpos3 * 0.031816465256797585_f64), perm_table_minecraft_gravel_1_octave__5) * 0.06666666666666667_f64));
            out[index(p)] = (n * 1.3333333333333333_f64);
        }
    }
    
    pub fn minecraft_cave_layer_75_d5x263x5(out: &mut [f64; 6575], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, perm_table_minecraft_cave_layer_0_octave__8: &PerlinNoiseSampler, perm_table_minecraft_cave_layer_1_octave__8: &PerlinNoiseSampler, host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut n: f64;
        for p in iter_3d(5_i32, 263_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * Vec3::new(0.5_f64, 0.5_f64, 0.5_f64)) + (pos3 * Vec3::new(2_f64, 4_f64, 2_f64)));
            n = 0_f64;
            n = (n + (perlin((rpos3 * 0.00390625_f64), perm_table_minecraft_cave_layer_0_octave__8) * 1_f64));
            n = (n + (perlin((rpos3 * 0.003977058157099698_f64), perm_table_minecraft_cave_layer_1_octave__8) * 1_f64));
            out[index(p)] = (n * 0.8333333333333333_f64);
        }
    }
    
    pub fn density_function_ShiftedNoise_50_d5x263x5(out: &mut [f64; 6575], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, perm_table_minecraft_gravel_0_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__5: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__5: &PerlinNoiseSampler, host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        for p in iter_3d(5_i32, 263_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            out[index(p)] = density_function_ShiftedNoise_50_d5x263x5_minecraft_gravel_shifted_51(((rpos3 * Vec3::new(0.25_f64, 0_f64, 0.25_f64)) + Vec3::new(1_f64, 0_f64, 0_f64)), perm_table_minecraft_gravel_0_octave__8, perm_table_minecraft_gravel_1_octave__8, perm_table_minecraft_gravel_0_octave__7, perm_table_minecraft_gravel_1_octave__7, perm_table_minecraft_gravel_0_octave__6, perm_table_minecraft_gravel_1_octave__6, perm_table_minecraft_gravel_0_octave__5, perm_table_minecraft_gravel_1_octave__5, host_consts);
        }
    }
    
    pub fn density_function_ShiftedNoise_25_d5x1x5(out: &mut [f64; 25], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, perm_table_minecraft_gravel_0_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__5: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__5: &PerlinNoiseSampler, host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        for p in iter_3d(5_i32, 1_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            out[index(p)] = density_function_ShiftedNoise_25_d5x1x5_minecraft_gravel_shifted_26(((rpos3 * Vec3::new(0.11_f64, 0_f64, 0.11_f64)) + Vec3::new(1_f64, 0_f64, 0_f64)), perm_table_minecraft_gravel_0_octave__8, perm_table_minecraft_gravel_1_octave__8, perm_table_minecraft_gravel_0_octave__7, perm_table_minecraft_gravel_1_octave__7, perm_table_minecraft_gravel_0_octave__6, perm_table_minecraft_gravel_1_octave__6, perm_table_minecraft_gravel_0_octave__5, perm_table_minecraft_gravel_1_octave__5, host_consts);
        }
    }
    
    pub fn minecraft_realism_extreme_hills_jagged_original_d5x1x5(out: &mut [f64; 25], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207175096: &[f64; 25], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        for p in iter_3d(5_i32, 1_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            out[index(p)] = input_135471207175096[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize];
        }
    }
    
    pub fn density_function_Add_45_d5x263x5(out: &mut [f64; 6575], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207190888: &[f64; 25], input_135471207190408: &[f64; 25], input_135471207189928: &[f64; 25], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        for p in iter_3d(5_i32, 263_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            out[index(p)] = (((input_135471207190888[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * 0.1_f64) + -0.8_f64) + (((input_135471207190408[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * 0_f64) + 0_f64) + ((input_135471207189928[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * 0.015_f64) + 0_f64)));
        }
    }
    
    pub fn density_function_Abs_38_d5x1x5(out: &mut [f64; 25], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207156304: &[f64; 25], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        for p in iter_3d(5_i32, 1_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            out[index(p)] = abs(input_135471207156304[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize]);
        }
    }
    
    pub fn minecraft_realism_mountains_jagged_original_d5x1x5(out: &mut [f64; 25], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207167544: &[f64; 25], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        for p in iter_3d(5_i32, 1_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            out[index(p)] = input_135471207167544[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize];
        }
    }
    
    pub fn minecraft_realism_extreme_hills_factor_original_d5x1x5(out: &mut [f64; 25], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207187312: &[f64; 25], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        for p in iter_3d(5_i32, 1_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            out[index(p)] = input_135471207187312[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize];
        }
    }
    
    pub fn minecraft_gravel_23_d5x1x5(out: &mut [f64; 25], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, perm_table_minecraft_gravel_0_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__5: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__5: &PerlinNoiseSampler, host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut n: f64;
        for p in iter_3d(5_i32, 1_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * Vec3::new(0.11_f64, 0_f64, 0.11_f64)) + (pos3 * Vec3::new(0.44_f64, 0_f64, 0.44_f64)));
            n = 0_f64;
            n = (n + (perlin((rpos3 * 0.00390625_f64), perm_table_minecraft_gravel_0_octave__8) * 0.5333333333333333_f64));
            n = (n + (perlin((rpos3 * 0.003977058157099698_f64), perm_table_minecraft_gravel_1_octave__8) * 0.5333333333333333_f64));
            n = (n + (perlin((rpos3 * 0.0078125_f64), perm_table_minecraft_gravel_0_octave__7) * 0.26666666666666666_f64));
            n = (n + (perlin((rpos3 * 0.007954116314199396_f64), perm_table_minecraft_gravel_1_octave__7) * 0.26666666666666666_f64));
            n = (n + (perlin((rpos3 * 0.015625_f64), perm_table_minecraft_gravel_0_octave__6) * 0.13333333333333333_f64));
            n = (n + (perlin((rpos3 * 0.015908232628398793_f64), perm_table_minecraft_gravel_1_octave__6) * 0.13333333333333333_f64));
            n = (n + (perlin((rpos3 * 0.03125_f64), perm_table_minecraft_gravel_0_octave__5) * 0.06666666666666667_f64));
            n = (n + (perlin((rpos3 * 0.031816465256797585_f64), perm_table_minecraft_gravel_1_octave__5) * 0.06666666666666667_f64));
            out[index(p)] = (n * 1.3333333333333333_f64);
        }
    }
    
    pub fn minecraft_cave_layer_7_d5x1x5(out: &mut [f64; 25], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, perm_table_minecraft_cave_layer_0_octave__8: &PerlinNoiseSampler, perm_table_minecraft_cave_layer_1_octave__8: &PerlinNoiseSampler, host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut n: f64;
        for p in iter_3d(5_i32, 1_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * Vec3::new(0.1_f64, 0_f64, 0.1_f64)) + (pos3 * Vec3::new(0.4_f64, 0_f64, 0.4_f64)));
            n = 0_f64;
            n = (n + (perlin((rpos3 * 0.00390625_f64), perm_table_minecraft_cave_layer_0_octave__8) * 1_f64));
            n = (n + (perlin((rpos3 * 0.003977058157099698_f64), perm_table_minecraft_cave_layer_1_octave__8) * 1_f64));
            out[index(p)] = (n * 0.8333333333333333_f64);
        }
    }
    
    pub fn minecraft_jagged_5_d5x1x5(out: &mut [f64; 25], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, perm_table_minecraft_jagged_0_octave__16: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__16: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__15: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__15: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__14: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__14: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__13: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__13: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__12: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__12: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__11: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__11: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__10: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__10: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__9: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__9: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__8: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__8: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__7: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__7: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__6: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__6: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__5: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__5: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__4: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__4: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__3: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__3: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__2: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__2: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__1: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__1: &PerlinNoiseSampler, host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut n: f64;
        for p in iter_3d(5_i32, 1_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * Vec3::new(1_f64, 0_f64, 1_f64)) + (pos3 * Vec3::new(4_f64, 0_f64, 4_f64)));
            n = 0_f64;
            n = (n + (perlin((rpos3 * 0.0000152587890625_f64), perm_table_minecraft_jagged_0_octave__16) * 0.5000076295109483_f64));
            n = (n + (perlin((rpos3 * 0.000015535383426170696_f64), perm_table_minecraft_jagged_1_octave__16) * 0.5000076295109483_f64));
            n = (n + (perlin((rpos3 * 0.000030517578125_f64), perm_table_minecraft_jagged_0_octave__15) * 0.2500038147554742_f64));
            n = (n + (perlin((rpos3 * 0.00003107076685234139_f64), perm_table_minecraft_jagged_1_octave__15) * 0.2500038147554742_f64));
            n = (n + (perlin((rpos3 * 0.00006103515625_f64), perm_table_minecraft_jagged_0_octave__14) * 0.1250019073777371_f64));
            n = (n + (perlin((rpos3 * 0.00006214153370468278_f64), perm_table_minecraft_jagged_1_octave__14) * 0.1250019073777371_f64));
            n = (n + (perlin((rpos3 * 0.0001220703125_f64), perm_table_minecraft_jagged_0_octave__13) * 0.06250095368886854_f64));
            n = (n + (perlin((rpos3 * 0.00012428306740936557_f64), perm_table_minecraft_jagged_1_octave__13) * 0.06250095368886854_f64));
            n = (n + (perlin((rpos3 * 0.000244140625_f64), perm_table_minecraft_jagged_0_octave__12) * 0.03125047684443427_f64));
            n = (n + (perlin((rpos3 * 0.00024856613481873114_f64), perm_table_minecraft_jagged_1_octave__12) * 0.03125047684443427_f64));
            n = (n + (perlin((rpos3 * 0.00048828125_f64), perm_table_minecraft_jagged_0_octave__11) * 0.015625238422217136_f64));
            n = (n + (perlin((rpos3 * 0.0004971322696374623_f64), perm_table_minecraft_jagged_1_octave__11) * 0.015625238422217136_f64));
            n = (n + (perlin((rpos3 * 0.0009765625_f64), perm_table_minecraft_jagged_0_octave__10) * 0.007812619211108568_f64));
            n = (n + (perlin((rpos3 * 0.0009942645392749245_f64), perm_table_minecraft_jagged_1_octave__10) * 0.007812619211108568_f64));
            n = (n + (perlin((rpos3 * 0.001953125_f64), perm_table_minecraft_jagged_0_octave__9) * 0.003906309605554284_f64));
            n = (n + (perlin((rpos3 * 0.001988529078549849_f64), perm_table_minecraft_jagged_1_octave__9) * 0.003906309605554284_f64));
            n = (n + (perlin((rpos3 * 0.00390625_f64), perm_table_minecraft_jagged_0_octave__8) * 0.001953154802777142_f64));
            n = (n + (perlin((rpos3 * 0.003977058157099698_f64), perm_table_minecraft_jagged_1_octave__8) * 0.001953154802777142_f64));
            n = (n + (perlin((rpos3 * 0.0078125_f64), perm_table_minecraft_jagged_0_octave__7) * 0.000976577401388571_f64));
            n = (n + (perlin((rpos3 * 0.007954116314199396_f64), perm_table_minecraft_jagged_1_octave__7) * 0.000976577401388571_f64));
            n = (n + (perlin((rpos3 * 0.015625_f64), perm_table_minecraft_jagged_0_octave__6) * 0.0004882887006942855_f64));
            n = (n + (perlin((rpos3 * 0.015908232628398793_f64), perm_table_minecraft_jagged_1_octave__6) * 0.0004882887006942855_f64));
            n = (n + (perlin((rpos3 * 0.03125_f64), perm_table_minecraft_jagged_0_octave__5) * 0.00024414435034714275_f64));
            n = (n + (perlin((rpos3 * 0.031816465256797585_f64), perm_table_minecraft_jagged_1_octave__5) * 0.00024414435034714275_f64));
            n = (n + (perlin((rpos3 * 0.0625_f64), perm_table_minecraft_jagged_0_octave__4) * 0.00012207217517357137_f64));
            n = (n + (perlin((rpos3 * 0.06363293051359517_f64), perm_table_minecraft_jagged_1_octave__4) * 0.00012207217517357137_f64));
            n = (n + (perlin((rpos3 * 0.125_f64), perm_table_minecraft_jagged_0_octave__3) * 0.00006103608758678569_f64));
            n = (n + (perlin((rpos3 * 0.12726586102719034_f64), perm_table_minecraft_jagged_1_octave__3) * 0.00006103608758678569_f64));
            n = (n + (perlin((rpos3 * 0.25_f64), perm_table_minecraft_jagged_0_octave__2) * 0.000030518043793392844_f64));
            n = (n + (perlin((rpos3 * 0.2545317220543807_f64), perm_table_minecraft_jagged_1_octave__2) * 0.000030518043793392844_f64));
            n = (n + (perlin((rpos3 * 0.5_f64), perm_table_minecraft_jagged_0_octave__1) * 0.000015259021896696422_f64));
            n = (n + (perlin((rpos3 * 0.5090634441087614_f64), perm_table_minecraft_jagged_1_octave__1) * 0.000015259021896696422_f64));
            out[index(p)] = (n * 1.568627450980392_f64);
        }
    }
    
    pub fn Beardifier_d16x2096x16(out: &mut [f64; 536576], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut ret_0: f64;
        let mut current_piece_1: Piece;
        let mut current_box_2: BlockBox;
        let mut current_jigsaw_3: Jigsaw;
        let mut i_4: i32;
        let mut m_5: i32;
        let mut n_6: i32;
        let mut o_7: i32;
        let mut p_8: i32;
        let mut q_9: i32;
        let mut s_10: i32;
        let mut r_11: i32;
        let mut l_12: i32;
        let mut m_13: i32;
        for p in iter_3d(16_i32, 2096_i32, 16_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            ret_0 = 0_f64;
            if do_beardify(host_consts.base_box, rpos3) {
                i_4 = 0_i32;
                while (i_4 < 500_i32) {
                    current_piece_1 = host_consts.piece_array[i_4 as usize];
                    current_box_2 = current_piece_1.block_box;
                    if (!valid_box(current_box_2)) {
                        break;
                    }
                    m_5 = (max(0_f64, max((current_box_2.min_x - rpos3.x), (rpos3.x - current_box_2.max_x))) as i32);
                    n_6 = (max(0_f64, max((current_box_2.min_z - rpos3.z), (rpos3.z - current_box_2.max_z))) as i32);
                    o_7 = ((current_box_2.min_y + (current_piece_1.ground_level_delta as f64)) as i32);
                    p_8 = ((rpos3.y - (o_7 as f64)) as i32);
                    s_10 = current_piece_1.terrain_adjustment;
                    q_9 = 0_i32;
                    if ((s_10 == 1_i32) || (s_10 == 2_i32)) {
                        q_9 = p_8;
                    }
                    if (s_10 == 3_i32) {
                        q_9 = (max(0_f64, max(((o_7 as f64) - rpos3.y), (rpos3.y - current_box_2.max_y))) as i32);
                    }
                    if (s_10 == 4_i32) {
                        q_9 = (max(0_f64, max((current_box_2.min_y - rpos3.y), (rpos3.y - current_box_2.max_y))) as i32);
                    }
                    if (s_10 == 1_i32) {
                        ret_0 = (ret_0 + get_magnitude_weight((m_5 as f64), ((q_9 as f64) / 2_f64), (n_6 as f64)));
                    }
                    if ((s_10 == 2_i32) || (s_10 == 3_i32)) {
                        ret_0 = (ret_0 + (get_structure_weight(m_5, q_9, n_6, p_8) * 0.8_f64));
                    }
                    if (s_10 == 4_i32) {
                        ret_0 = (ret_0 + (get_magnitude_weight(((m_5 as f64) / 2_f64), ((q_9 as f64) / 2_f64), ((n_6 as f64) / 2_f64)) * 0.8_f64));
                    }
                    i_4 = (i_4 + 1_i32);
                }
                i_4 = 0_i32;
                while (i_4 < 500_i32) {
                    current_jigsaw_3 = host_consts.jigsaw_array[i_4 as usize];
                    if (((current_jigsaw_3.source_x == -1073741824_i32) && (current_jigsaw_3.source_ground_y == -1073741824_i32)) && (current_jigsaw_3.source_z == -1073741824_i32)) {
                        break;
                    }
                    r_11 = ((rpos3.x as i32) - current_jigsaw_3.source_x);
                    l_12 = ((rpos3.y as i32) - current_jigsaw_3.source_ground_y);
                    m_13 = ((rpos3.z as i32) - current_jigsaw_3.source_z);
                    ret_0 = (ret_0 + (get_structure_weight(r_11, l_12, m_13, l_12) * 0.4_f64));
                    i_4 = (i_4 + 1_i32);
                }
            }
            out[index(p)] = ret_0;
        }
    }
    
    pub fn minecraft_caves_barrier_2_d5x263x5(out: &mut [f64; 6575], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207106704: &[f64; 6575], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        for p in iter_3d(5_i32, 263_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            out[index(p)] = input_135471207106704[index(p) as usize];
        }
    }
    
    pub fn minecraft_jagged_3_d5x1x5(out: &mut [f64; 25], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, perm_table_minecraft_jagged_0_octave__16: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__16: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__15: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__15: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__14: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__14: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__13: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__13: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__12: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__12: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__11: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__11: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__10: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__10: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__9: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__9: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__8: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__8: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__7: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__7: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__6: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__6: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__5: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__5: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__4: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__4: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__3: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__3: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__2: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__2: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__1: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__1: &PerlinNoiseSampler, host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut n: f64;
        for p in iter_3d(5_i32, 1_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * Vec3::new(175_f64, 0_f64, 175_f64)) + (pos3 * Vec3::new(700_f64, 0_f64, 700_f64)));
            n = 0_f64;
            n = (n + (perlin((rpos3 * 0.0000152587890625_f64), perm_table_minecraft_jagged_0_octave__16) * 0.5000076295109483_f64));
            n = (n + (perlin((rpos3 * 0.000015535383426170696_f64), perm_table_minecraft_jagged_1_octave__16) * 0.5000076295109483_f64));
            n = (n + (perlin((rpos3 * 0.000030517578125_f64), perm_table_minecraft_jagged_0_octave__15) * 0.2500038147554742_f64));
            n = (n + (perlin((rpos3 * 0.00003107076685234139_f64), perm_table_minecraft_jagged_1_octave__15) * 0.2500038147554742_f64));
            n = (n + (perlin((rpos3 * 0.00006103515625_f64), perm_table_minecraft_jagged_0_octave__14) * 0.1250019073777371_f64));
            n = (n + (perlin((rpos3 * 0.00006214153370468278_f64), perm_table_minecraft_jagged_1_octave__14) * 0.1250019073777371_f64));
            n = (n + (perlin((rpos3 * 0.0001220703125_f64), perm_table_minecraft_jagged_0_octave__13) * 0.06250095368886854_f64));
            n = (n + (perlin((rpos3 * 0.00012428306740936557_f64), perm_table_minecraft_jagged_1_octave__13) * 0.06250095368886854_f64));
            n = (n + (perlin((rpos3 * 0.000244140625_f64), perm_table_minecraft_jagged_0_octave__12) * 0.03125047684443427_f64));
            n = (n + (perlin((rpos3 * 0.00024856613481873114_f64), perm_table_minecraft_jagged_1_octave__12) * 0.03125047684443427_f64));
            n = (n + (perlin((rpos3 * 0.00048828125_f64), perm_table_minecraft_jagged_0_octave__11) * 0.015625238422217136_f64));
            n = (n + (perlin((rpos3 * 0.0004971322696374623_f64), perm_table_minecraft_jagged_1_octave__11) * 0.015625238422217136_f64));
            n = (n + (perlin((rpos3 * 0.0009765625_f64), perm_table_minecraft_jagged_0_octave__10) * 0.007812619211108568_f64));
            n = (n + (perlin((rpos3 * 0.0009942645392749245_f64), perm_table_minecraft_jagged_1_octave__10) * 0.007812619211108568_f64));
            n = (n + (perlin((rpos3 * 0.001953125_f64), perm_table_minecraft_jagged_0_octave__9) * 0.003906309605554284_f64));
            n = (n + (perlin((rpos3 * 0.001988529078549849_f64), perm_table_minecraft_jagged_1_octave__9) * 0.003906309605554284_f64));
            n = (n + (perlin((rpos3 * 0.00390625_f64), perm_table_minecraft_jagged_0_octave__8) * 0.001953154802777142_f64));
            n = (n + (perlin((rpos3 * 0.003977058157099698_f64), perm_table_minecraft_jagged_1_octave__8) * 0.001953154802777142_f64));
            n = (n + (perlin((rpos3 * 0.0078125_f64), perm_table_minecraft_jagged_0_octave__7) * 0.000976577401388571_f64));
            n = (n + (perlin((rpos3 * 0.007954116314199396_f64), perm_table_minecraft_jagged_1_octave__7) * 0.000976577401388571_f64));
            n = (n + (perlin((rpos3 * 0.015625_f64), perm_table_minecraft_jagged_0_octave__6) * 0.0004882887006942855_f64));
            n = (n + (perlin((rpos3 * 0.015908232628398793_f64), perm_table_minecraft_jagged_1_octave__6) * 0.0004882887006942855_f64));
            n = (n + (perlin((rpos3 * 0.03125_f64), perm_table_minecraft_jagged_0_octave__5) * 0.00024414435034714275_f64));
            n = (n + (perlin((rpos3 * 0.031816465256797585_f64), perm_table_minecraft_jagged_1_octave__5) * 0.00024414435034714275_f64));
            n = (n + (perlin((rpos3 * 0.0625_f64), perm_table_minecraft_jagged_0_octave__4) * 0.00012207217517357137_f64));
            n = (n + (perlin((rpos3 * 0.06363293051359517_f64), perm_table_minecraft_jagged_1_octave__4) * 0.00012207217517357137_f64));
            n = (n + (perlin((rpos3 * 0.125_f64), perm_table_minecraft_jagged_0_octave__3) * 0.00006103608758678569_f64));
            n = (n + (perlin((rpos3 * 0.12726586102719034_f64), perm_table_minecraft_jagged_1_octave__3) * 0.00006103608758678569_f64));
            n = (n + (perlin((rpos3 * 0.25_f64), perm_table_minecraft_jagged_0_octave__2) * 0.000030518043793392844_f64));
            n = (n + (perlin((rpos3 * 0.2545317220543807_f64), perm_table_minecraft_jagged_1_octave__2) * 0.000030518043793392844_f64));
            n = (n + (perlin((rpos3 * 0.5_f64), perm_table_minecraft_jagged_0_octave__1) * 0.000015259021896696422_f64));
            n = (n + (perlin((rpos3 * 0.5090634441087614_f64), perm_table_minecraft_jagged_1_octave__1) * 0.000015259021896696422_f64));
            out[index(p)] = (n * 1.568627450980392_f64);
        }
    }
    
    pub fn density_function_ShiftedNoise_56_d5x263x5(out: &mut [f64; 6575], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, perm_table_minecraft_gravel_0_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__5: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__5: &PerlinNoiseSampler, host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        for p in iter_3d(5_i32, 263_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            out[index(p)] = density_function_ShiftedNoise_56_d5x263x5_minecraft_gravel_shifted_57(((rpos3 * Vec3::new(0.11_f64, 0_f64, 0.11_f64)) + Vec3::new(1_f64, 0_f64, 0_f64)), perm_table_minecraft_gravel_0_octave__8, perm_table_minecraft_gravel_1_octave__8, perm_table_minecraft_gravel_0_octave__7, perm_table_minecraft_gravel_1_octave__7, perm_table_minecraft_gravel_0_octave__6, perm_table_minecraft_gravel_1_octave__6, perm_table_minecraft_gravel_0_octave__5, perm_table_minecraft_gravel_1_octave__5, host_consts);
        }
    }
    
    pub fn minecraft_gravel_31_d5x1x5(out: &mut [f64; 25], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, perm_table_minecraft_gravel_0_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__5: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__5: &PerlinNoiseSampler, host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut n: f64;
        for p in iter_3d(5_i32, 1_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * Vec3::new(0.12_f64, 0_f64, 0.12_f64)) + (pos3 * Vec3::new(0.48_f64, 0_f64, 0.48_f64)));
            n = 0_f64;
            n = (n + (perlin((rpos3 * 0.00390625_f64), perm_table_minecraft_gravel_0_octave__8) * 0.5333333333333333_f64));
            n = (n + (perlin((rpos3 * 0.003977058157099698_f64), perm_table_minecraft_gravel_1_octave__8) * 0.5333333333333333_f64));
            n = (n + (perlin((rpos3 * 0.0078125_f64), perm_table_minecraft_gravel_0_octave__7) * 0.26666666666666666_f64));
            n = (n + (perlin((rpos3 * 0.007954116314199396_f64), perm_table_minecraft_gravel_1_octave__7) * 0.26666666666666666_f64));
            n = (n + (perlin((rpos3 * 0.015625_f64), perm_table_minecraft_gravel_0_octave__6) * 0.13333333333333333_f64));
            n = (n + (perlin((rpos3 * 0.015908232628398793_f64), perm_table_minecraft_gravel_1_octave__6) * 0.13333333333333333_f64));
            n = (n + (perlin((rpos3 * 0.03125_f64), perm_table_minecraft_gravel_0_octave__5) * 0.06666666666666667_f64));
            n = (n + (perlin((rpos3 * 0.031816465256797585_f64), perm_table_minecraft_gravel_1_octave__5) * 0.06666666666666667_f64));
            out[index(p)] = (n * 1.3333333333333333_f64);
        }
    }
    
    pub fn minecraft_jagged_21_d5x1x5(out: &mut [f64; 25], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, perm_table_minecraft_jagged_0_octave__16: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__16: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__15: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__15: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__14: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__14: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__13: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__13: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__12: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__12: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__11: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__11: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__10: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__10: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__9: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__9: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__8: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__8: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__7: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__7: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__6: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__6: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__5: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__5: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__4: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__4: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__3: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__3: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__2: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__2: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__1: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__1: &PerlinNoiseSampler, host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut n: f64;
        for p in iter_3d(5_i32, 1_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * Vec3::new(300_f64, 0_f64, 300_f64)) + (pos3 * Vec3::new(1200_f64, 0_f64, 1200_f64)));
            n = 0_f64;
            n = (n + (perlin((rpos3 * 0.0000152587890625_f64), perm_table_minecraft_jagged_0_octave__16) * 0.5000076295109483_f64));
            n = (n + (perlin((rpos3 * 0.000015535383426170696_f64), perm_table_minecraft_jagged_1_octave__16) * 0.5000076295109483_f64));
            n = (n + (perlin((rpos3 * 0.000030517578125_f64), perm_table_minecraft_jagged_0_octave__15) * 0.2500038147554742_f64));
            n = (n + (perlin((rpos3 * 0.00003107076685234139_f64), perm_table_minecraft_jagged_1_octave__15) * 0.2500038147554742_f64));
            n = (n + (perlin((rpos3 * 0.00006103515625_f64), perm_table_minecraft_jagged_0_octave__14) * 0.1250019073777371_f64));
            n = (n + (perlin((rpos3 * 0.00006214153370468278_f64), perm_table_minecraft_jagged_1_octave__14) * 0.1250019073777371_f64));
            n = (n + (perlin((rpos3 * 0.0001220703125_f64), perm_table_minecraft_jagged_0_octave__13) * 0.06250095368886854_f64));
            n = (n + (perlin((rpos3 * 0.00012428306740936557_f64), perm_table_minecraft_jagged_1_octave__13) * 0.06250095368886854_f64));
            n = (n + (perlin((rpos3 * 0.000244140625_f64), perm_table_minecraft_jagged_0_octave__12) * 0.03125047684443427_f64));
            n = (n + (perlin((rpos3 * 0.00024856613481873114_f64), perm_table_minecraft_jagged_1_octave__12) * 0.03125047684443427_f64));
            n = (n + (perlin((rpos3 * 0.00048828125_f64), perm_table_minecraft_jagged_0_octave__11) * 0.015625238422217136_f64));
            n = (n + (perlin((rpos3 * 0.0004971322696374623_f64), perm_table_minecraft_jagged_1_octave__11) * 0.015625238422217136_f64));
            n = (n + (perlin((rpos3 * 0.0009765625_f64), perm_table_minecraft_jagged_0_octave__10) * 0.007812619211108568_f64));
            n = (n + (perlin((rpos3 * 0.0009942645392749245_f64), perm_table_minecraft_jagged_1_octave__10) * 0.007812619211108568_f64));
            n = (n + (perlin((rpos3 * 0.001953125_f64), perm_table_minecraft_jagged_0_octave__9) * 0.003906309605554284_f64));
            n = (n + (perlin((rpos3 * 0.001988529078549849_f64), perm_table_minecraft_jagged_1_octave__9) * 0.003906309605554284_f64));
            n = (n + (perlin((rpos3 * 0.00390625_f64), perm_table_minecraft_jagged_0_octave__8) * 0.001953154802777142_f64));
            n = (n + (perlin((rpos3 * 0.003977058157099698_f64), perm_table_minecraft_jagged_1_octave__8) * 0.001953154802777142_f64));
            n = (n + (perlin((rpos3 * 0.0078125_f64), perm_table_minecraft_jagged_0_octave__7) * 0.000976577401388571_f64));
            n = (n + (perlin((rpos3 * 0.007954116314199396_f64), perm_table_minecraft_jagged_1_octave__7) * 0.000976577401388571_f64));
            n = (n + (perlin((rpos3 * 0.015625_f64), perm_table_minecraft_jagged_0_octave__6) * 0.0004882887006942855_f64));
            n = (n + (perlin((rpos3 * 0.015908232628398793_f64), perm_table_minecraft_jagged_1_octave__6) * 0.0004882887006942855_f64));
            n = (n + (perlin((rpos3 * 0.03125_f64), perm_table_minecraft_jagged_0_octave__5) * 0.00024414435034714275_f64));
            n = (n + (perlin((rpos3 * 0.031816465256797585_f64), perm_table_minecraft_jagged_1_octave__5) * 0.00024414435034714275_f64));
            n = (n + (perlin((rpos3 * 0.0625_f64), perm_table_minecraft_jagged_0_octave__4) * 0.00012207217517357137_f64));
            n = (n + (perlin((rpos3 * 0.06363293051359517_f64), perm_table_minecraft_jagged_1_octave__4) * 0.00012207217517357137_f64));
            n = (n + (perlin((rpos3 * 0.125_f64), perm_table_minecraft_jagged_0_octave__3) * 0.00006103608758678569_f64));
            n = (n + (perlin((rpos3 * 0.12726586102719034_f64), perm_table_minecraft_jagged_1_octave__3) * 0.00006103608758678569_f64));
            n = (n + (perlin((rpos3 * 0.25_f64), perm_table_minecraft_jagged_0_octave__2) * 0.000030518043793392844_f64));
            n = (n + (perlin((rpos3 * 0.2545317220543807_f64), perm_table_minecraft_jagged_1_octave__2) * 0.000030518043793392844_f64));
            n = (n + (perlin((rpos3 * 0.5_f64), perm_table_minecraft_jagged_0_octave__1) * 0.000015259021896696422_f64));
            n = (n + (perlin((rpos3 * 0.5090634441087614_f64), perm_table_minecraft_jagged_1_octave__1) * 0.000015259021896696422_f64));
            out[index(p)] = (n * 1.568627450980392_f64);
        }
    }
    
    pub fn minecraft_cave_layer_66_d5x263x5(out: &mut [f64; 6575], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, perm_table_minecraft_cave_layer_0_octave__8: &PerlinNoiseSampler, perm_table_minecraft_cave_layer_1_octave__8: &PerlinNoiseSampler, host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut n: f64;
        for p in iter_3d(5_i32, 263_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * Vec3::new(8_f64, 8_f64, 8_f64)) + (pos3 * Vec3::new(32_f64, 64_f64, 32_f64)));
            n = 0_f64;
            n = (n + (perlin((rpos3 * 0.00390625_f64), perm_table_minecraft_cave_layer_0_octave__8) * 1_f64));
            n = (n + (perlin((rpos3 * 0.003977058157099698_f64), perm_table_minecraft_cave_layer_1_octave__8) * 1_f64));
            out[index(p)] = (n * 0.8333333333333333_f64);
        }
    }
    
    pub fn minecraft_realism_extreme_hills_jagged_original_d5x263x5(out: &mut [f64; 6575], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207175096: &[f64; 25], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        for p in iter_3d(5_i32, 263_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            out[index(p)] = input_135471207175096[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize];
        }
    }
    
    pub fn density_function_Add_46_d5x263x5(out: &mut [f64; 6575], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207194440: &[f64; 25], input_135471207193960: &[f64; 25], input_135471207193480: &[f64; 25], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        for p in iter_3d(5_i32, 263_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            out[index(p)] = (((input_135471207194440[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * 0.09_f64) + -0.915_f64) + (((input_135471207193960[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * 0.05_f64) + 0_f64) + ((input_135471207193480[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * 0.015_f64) + 0_f64)));
        }
    }
    
    pub fn minecraft_realism_extreme_mountains_jagged_original_d5x1x5(out: &mut [f64; 25], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207159400: &[f64; 25], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        for p in iter_3d(5_i32, 1_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            out[index(p)] = input_135471207159400[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize];
        }
    }
    
    pub fn minecraft_realism_mountains_factor_original_d5x263x5(out: &mut [f64; 6575], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207172112: &[f64; 25], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        for p in iter_3d(5_i32, 263_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            out[index(p)] = input_135471207172112[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize];
        }
    }
    
    pub fn minecraft_gravel_2_d5x1x5(out: &mut [f64; 25], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, perm_table_minecraft_gravel_0_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__5: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__5: &PerlinNoiseSampler, host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut n: f64;
        for p in iter_3d(5_i32, 1_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * Vec3::new(0.05_f64, 0_f64, 0.05_f64)) + (pos3 * Vec3::new(0.2_f64, 0_f64, 0.2_f64)));
            n = 0_f64;
            n = (n + (perlin((rpos3 * 0.00390625_f64), perm_table_minecraft_gravel_0_octave__8) * 0.5333333333333333_f64));
            n = (n + (perlin((rpos3 * 0.003977058157099698_f64), perm_table_minecraft_gravel_1_octave__8) * 0.5333333333333333_f64));
            n = (n + (perlin((rpos3 * 0.0078125_f64), perm_table_minecraft_gravel_0_octave__7) * 0.26666666666666666_f64));
            n = (n + (perlin((rpos3 * 0.007954116314199396_f64), perm_table_minecraft_gravel_1_octave__7) * 0.26666666666666666_f64));
            n = (n + (perlin((rpos3 * 0.015625_f64), perm_table_minecraft_gravel_0_octave__6) * 0.13333333333333333_f64));
            n = (n + (perlin((rpos3 * 0.015908232628398793_f64), perm_table_minecraft_gravel_1_octave__6) * 0.13333333333333333_f64));
            n = (n + (perlin((rpos3 * 0.03125_f64), perm_table_minecraft_gravel_0_octave__5) * 0.06666666666666667_f64));
            n = (n + (perlin((rpos3 * 0.031816465256797585_f64), perm_table_minecraft_gravel_1_octave__5) * 0.06666666666666667_f64));
            out[index(p)] = (n * 1.3333333333333333_f64);
        }
    }
    
    pub fn minecraft_realism_extreme_hills_factor_divergence_squared_d5x1x5(out: &mut [f64; 25], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207183992: &[f64; 25], input_135471207187312: &[f64; 25], input_135471207182664: &[f64; 25], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut var_0: f64;
        let mut var_1: f64;
        for p in iter_3d(5_i32, 1_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            var_0 = (input_135471207183992[index(p) as usize] + (input_135471207187312[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * -1_f64));
            var_1 = (input_135471207182664[index(p) as usize] + (input_135471207187312[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * -1_f64));
            out[index(p)] = (((var_0 * var_0) + (var_1 * var_1)) * 10000_f64);
        }
    }
    
    pub fn minecraft_realism_mountains_slope_height_d5x1x5(out: &mut [f64; 25], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207172592: &[f64; 25], input_135471207172112: &[f64; 25], input_135471207168696: &[f64; 25], input_135471207167288: &[f64; 25], input_135471207166136: &[f64; 25], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut spline_values_0: [f32; 17];
        let mut coordinate_1: f32;
        let mut spline_index_2: i32;
        let mut spline_result_f32_3: f32;
        let mut spline_result_f64_4: f64;
        let mut spline_values_5: [f32; 1];
        let mut coordinate_6: f32;
        let mut spline_index_7: i32;
        let mut spline_values_8: [f32; 1];
        let mut coordinate_9: f32;
        let mut spline_index_10: i32;
        let mut spline_values_11: [f32; 2];
        let mut coordinate_12: f32;
        let mut spline_index_13: i32;
        let mut spline_result_f32_14: f32;
        let mut spline_result_f64_15: f64;
        for p in iter_3d(5_i32, 1_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            spline_values_0 = [1_f32, 0.95238096_f32, 0.90909094_f32, 0.8333333_f32, 0.7692308_f32, 0.71428573_f32, 0.6666667_f32, 0.625_f32, 0.5882353_f32, 0.5555556_f32, 0.5263158_f32, 0.5_f32, 0.47619048_f32, 0.45454547_f32, 0.4347826_f32, 0.41666666_f32, 0.4_f32];
            coordinate_1 = (input_135471207168696[index(p) as usize] as f32);
            spline_index_2 = binary_search(minecraft_realism_mountains_slope_height_spline_coordinates_135471207172000, (coordinate_1 as f32));
            spline_result_f32_3 = advanced_hermite(minecraft_realism_mountains_slope_height_spline_coordinates_135471207172000, spline_values_0, minecraft_realism_mountains_slope_height_spline_derivatives_135471207171944, (coordinate_1 as f32), (spline_index_2 as i32));
            spline_result_f64_4 = (spline_result_f32_3 as f64);
            spline_values_5 = [0_f32];
            coordinate_6 = (input_135471207167288[index(p) as usize] as f32);
            spline_index_7 = binary_search(minecraft_realism_mountains_slope_height_spline_coordinates_135471207168472, (coordinate_6 as f32));
            spline_values_8 = [0_f32];
            coordinate_9 = (input_135471207167288[index(p) as usize] as f32);
            spline_index_10 = binary_search(minecraft_realism_mountains_slope_height_spline_coordinates_135471207167176, (coordinate_9 as f32));
            spline_values_11 = [advanced_hermite(minecraft_realism_mountains_slope_height_spline_coordinates_135471207168472, spline_values_5, minecraft_realism_mountains_slope_height_spline_derivatives_135471207168416, (coordinate_6 as f32), (spline_index_7 as i32)), advanced_hermite(minecraft_realism_mountains_slope_height_spline_coordinates_135471207167176, spline_values_8, minecraft_realism_mountains_slope_height_spline_derivatives_135471207167120, (coordinate_9 as f32), (spline_index_10 as i32))];
            coordinate_12 = (input_135471207166136[index(p) as usize] as f32);
            spline_index_13 = binary_search(minecraft_realism_mountains_slope_height_spline_coordinates_135471207166896, (coordinate_12 as f32));
            spline_result_f32_14 = advanced_hermite(minecraft_realism_mountains_slope_height_spline_coordinates_135471207166896, spline_values_11, minecraft_realism_mountains_slope_height_spline_derivatives_135471207166840, (coordinate_12 as f32), (spline_index_13 as i32));
            spline_result_f64_15 = (spline_result_f32_14 as f64);
            out[index(p)] = ((0_f64 + ((input_135471207172592[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * 0.2_f64) + 0_f64)) + (((input_135471207172112[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * spline_result_f64_4) * 0.7_f64) + spline_result_f64_15));
        }
    }
    
    pub fn minecraft_realism_mountains_slope_height_d5x263x5(out: &mut [f64; 6575], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207172592: &[f64; 25], input_135471207172112: &[f64; 25], input_135471207123504: &[f64; 6575], input_135471207122520: &[f64; 6575], input_135471207121368: &[f64; 6575], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut spline_values_0: [f32; 17];
        let mut coordinate_1: f32;
        let mut spline_index_2: i32;
        let mut spline_result_f32_3: f32;
        let mut spline_result_f64_4: f64;
        let mut spline_values_5: [f32; 1];
        let mut coordinate_6: f32;
        let mut spline_index_7: i32;
        let mut spline_values_8: [f32; 1];
        let mut coordinate_9: f32;
        let mut spline_index_10: i32;
        let mut spline_values_11: [f32; 2];
        let mut coordinate_12: f32;
        let mut spline_index_13: i32;
        let mut spline_result_f32_14: f32;
        let mut spline_result_f64_15: f64;
        for p in iter_3d(5_i32, 263_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            spline_values_0 = [1_f32, 0.95238096_f32, 0.90909094_f32, 0.8333333_f32, 0.7692308_f32, 0.71428573_f32, 0.6666667_f32, 0.625_f32, 0.5882353_f32, 0.5555556_f32, 0.5263158_f32, 0.5_f32, 0.47619048_f32, 0.45454547_f32, 0.4347826_f32, 0.41666666_f32, 0.4_f32];
            coordinate_1 = (input_135471207123504[index(p) as usize] as f32);
            spline_index_2 = binary_search(minecraft_realism_mountains_slope_height_spline_coordinates_135471207126808, (coordinate_1 as f32));
            spline_result_f32_3 = advanced_hermite(minecraft_realism_mountains_slope_height_spline_coordinates_135471207126808, spline_values_0, minecraft_realism_mountains_slope_height_spline_derivatives_135471207126752, (coordinate_1 as f32), (spline_index_2 as i32));
            spline_result_f64_4 = (spline_result_f32_3 as f64);
            spline_values_5 = [0_f32];
            coordinate_6 = (input_135471207122520[index(p) as usize] as f32);
            spline_index_7 = binary_search(minecraft_realism_mountains_slope_height_spline_coordinates_135471207123280, (coordinate_6 as f32));
            spline_values_8 = [0_f32];
            coordinate_9 = (input_135471207122520[index(p) as usize] as f32);
            spline_index_10 = binary_search(minecraft_realism_mountains_slope_height_spline_coordinates_135471207122408, (coordinate_9 as f32));
            spline_values_11 = [advanced_hermite(minecraft_realism_mountains_slope_height_spline_coordinates_135471207123280, spline_values_5, minecraft_realism_mountains_slope_height_spline_derivatives_135471207123224, (coordinate_6 as f32), (spline_index_7 as i32)), advanced_hermite(minecraft_realism_mountains_slope_height_spline_coordinates_135471207122408, spline_values_8, minecraft_realism_mountains_slope_height_spline_derivatives_135471207122352, (coordinate_9 as f32), (spline_index_10 as i32))];
            coordinate_12 = (input_135471207121368[index(p) as usize] as f32);
            spline_index_13 = binary_search(minecraft_realism_mountains_slope_height_spline_coordinates_135471207122128, (coordinate_12 as f32));
            spline_result_f32_14 = advanced_hermite(minecraft_realism_mountains_slope_height_spline_coordinates_135471207122128, spline_values_11, minecraft_realism_mountains_slope_height_spline_derivatives_135471207122072, (coordinate_12 as f32), (spline_index_13 as i32));
            spline_result_f64_15 = (spline_result_f32_14 as f64);
            out[index(p)] = ((0_f64 + ((input_135471207172592[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * 0.2_f64) + 0_f64)) + (((input_135471207172112[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * spline_result_f64_4) * 0.7_f64) + spline_result_f64_15));
        }
    }
    
    pub fn density_function_ShiftedNoise_63_d5x263x5(out: &mut [f64; 6575], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, perm_table_minecraft_gravel_0_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__5: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__5: &PerlinNoiseSampler, host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        for p in iter_3d(5_i32, 263_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            out[index(p)] = density_function_ShiftedNoise_63_d5x263x5_minecraft_gravel_shifted_64(((rpos3 * Vec3::new(0.12_f64, 0_f64, 0.12_f64)) + Vec3::new(0_f64, 0_f64, 1_f64)), perm_table_minecraft_gravel_0_octave__8, perm_table_minecraft_gravel_1_octave__8, perm_table_minecraft_gravel_0_octave__7, perm_table_minecraft_gravel_1_octave__7, perm_table_minecraft_gravel_0_octave__6, perm_table_minecraft_gravel_1_octave__6, perm_table_minecraft_gravel_0_octave__5, perm_table_minecraft_gravel_1_octave__5, host_consts);
        }
    }
    
    pub fn density_function_Add_6_d5x1x5(out: &mut [f64; 25], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207190888: &[f64; 25], input_135471207190408: &[f64; 25], input_135471207189928: &[f64; 25], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        for p in iter_3d(5_i32, 1_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            out[index(p)] = (((input_135471207190888[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * 0.1_f64) + -0.8_f64) + (((input_135471207190408[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * 0_f64) + 0_f64) + ((input_135471207189928[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * 0.015_f64) + 0_f64)));
        }
    }
    
    pub fn density_function_Multiply_60_d5x263x5(out: &mut [f64; 6575], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207118160: &[f64; 6575], input_135471207163968: &[f64; 25], input_135471207116832: &[f64; 6575], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut var_0: f64;
        let mut var_1: f64;
        for p in iter_3d(5_i32, 263_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            var_0 = (input_135471207118160[index(p) as usize] + (input_135471207163968[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * -1_f64));
            var_1 = (input_135471207116832[index(p) as usize] + (input_135471207163968[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * -1_f64));
            out[index(p)] = ((((var_0 * var_0) + (var_1 * var_1)) * 5000_f64) * 0.2_f64);
        }
    }
    
    pub fn minecraft_realism_mountains_jagged_original_d5x263x5(out: &mut [f64; 6575], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207167544: &[f64; 25], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        for p in iter_3d(5_i32, 263_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            out[index(p)] = input_135471207167544[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize];
        }
    }
    
    pub fn minecraft_caves_new_large_d5x263x5(out: &mut [f64; 6575], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207100120: &[f64; 6575], input_135471207094944: &[f64; 6575], input_135471207180296: &[f64; 25], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut spline_values_0: [f32; 41];
        let mut coordinate_1: f32;
        let mut spline_index_2: i32;
        let mut spline_result_f32_3: f32;
        let mut spline_result_f64_4: f64;
        for p in iter_3d(5_i32, 263_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            spline_values_0 = [1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32];
            coordinate_1 = (input_135471207100120[index(p) as usize] as f32);
            spline_index_2 = binary_search(minecraft_caves_new_large_spline_coordinates_135471207095760, (coordinate_1 as f32));
            spline_result_f32_3 = advanced_hermite(minecraft_caves_new_large_spline_coordinates_135471207095760, spline_values_0, minecraft_caves_new_large_spline_derivatives_135471207095704, (coordinate_1 as f32), (spline_index_2 as i32));
            spline_result_f64_4 = (spline_result_f32_3 as f64);
            out[index(p)] = (spline_result_f64_4 + ((input_135471207094944[index(p) as usize] * 1.5_f64) + (input_135471207180296[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * 0.5_f64)));
        }
    }
    
    pub fn minecraft_gravel_8_d5x1x5(out: &mut [f64; 25], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, perm_table_minecraft_gravel_0_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__5: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__5: &PerlinNoiseSampler, host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut n: f64;
        for p in iter_3d(5_i32, 1_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * Vec3::new(0.1_f64, 0_f64, 0.1_f64)) + (pos3 * Vec3::new(0.4_f64, 0_f64, 0.4_f64)));
            n = 0_f64;
            n = (n + (perlin((rpos3 * 0.00390625_f64), perm_table_minecraft_gravel_0_octave__8) * 0.5333333333333333_f64));
            n = (n + (perlin((rpos3 * 0.003977058157099698_f64), perm_table_minecraft_gravel_1_octave__8) * 0.5333333333333333_f64));
            n = (n + (perlin((rpos3 * 0.0078125_f64), perm_table_minecraft_gravel_0_octave__7) * 0.26666666666666666_f64));
            n = (n + (perlin((rpos3 * 0.007954116314199396_f64), perm_table_minecraft_gravel_1_octave__7) * 0.26666666666666666_f64));
            n = (n + (perlin((rpos3 * 0.015625_f64), perm_table_minecraft_gravel_0_octave__6) * 0.13333333333333333_f64));
            n = (n + (perlin((rpos3 * 0.015908232628398793_f64), perm_table_minecraft_gravel_1_octave__6) * 0.13333333333333333_f64));
            n = (n + (perlin((rpos3 * 0.03125_f64), perm_table_minecraft_gravel_0_octave__5) * 0.06666666666666667_f64));
            n = (n + (perlin((rpos3 * 0.031816465256797585_f64), perm_table_minecraft_gravel_1_octave__5) * 0.06666666666666667_f64));
            out[index(p)] = (n * 1.3333333333333333_f64);
        }
    }
    
    pub fn minecraft_caves_old_large_d5x263x5(out: &mut [f64; 6575], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207103832: &[f64; 6575], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        for p in iter_3d(5_i32, 263_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            out[index(p)] = input_135471207103832[index(p) as usize];
        }
    }
    
    pub fn minecraft_cave_layer_68_d5x263x5(out: &mut [f64; 6575], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, perm_table_minecraft_cave_layer_0_octave__8: &PerlinNoiseSampler, perm_table_minecraft_cave_layer_1_octave__8: &PerlinNoiseSampler, host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut n: f64;
        for p in iter_3d(5_i32, 263_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * Vec3::new(6_f64, 6_f64, 6_f64)) + (pos3 * Vec3::new(24_f64, 48_f64, 24_f64)));
            n = 0_f64;
            n = (n + (perlin((rpos3 * 0.00390625_f64), perm_table_minecraft_cave_layer_0_octave__8) * 1_f64));
            n = (n + (perlin((rpos3 * 0.003977058157099698_f64), perm_table_minecraft_cave_layer_1_octave__8) * 1_f64));
            out[index(p)] = (n * 0.8333333333333333_f64);
        }
    }
    
    pub fn density_function_ShiftedNoise_16_d5x1x5(out: &mut [f64; 25], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, perm_table_minecraft_gravel_0_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__5: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__5: &PerlinNoiseSampler, host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        for p in iter_3d(5_i32, 1_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            out[index(p)] = density_function_ShiftedNoise_16_d5x1x5_minecraft_gravel_shifted_17(((rpos3 * Vec3::new(0.25_f64, 0_f64, 0.25_f64)) + Vec3::new(0_f64, 0_f64, 1_f64)), perm_table_minecraft_gravel_0_octave__8, perm_table_minecraft_gravel_1_octave__8, perm_table_minecraft_gravel_0_octave__7, perm_table_minecraft_gravel_1_octave__7, perm_table_minecraft_gravel_0_octave__6, perm_table_minecraft_gravel_1_octave__6, perm_table_minecraft_gravel_0_octave__5, perm_table_minecraft_gravel_1_octave__5, host_consts);
        }
    }
    
    pub fn density_function_Multiply_49_d5x263x5(out: &mut [f64; 6575], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207135152: &[f64; 6575], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut spline_values_0: [f32; 20];
        let mut coordinate_1: f32;
        let mut spline_index_2: i32;
        let mut spline_result_f32_3: f32;
        let mut spline_result_f64_4: f64;
        for p in iter_3d(5_i32, 263_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            spline_values_0 = [0_f32, 0.31622776_f32, 0.4472136_f32, 0.5477226_f32, 0.6324555_f32, 0.70710677_f32, 0.7745967_f32, 0.83666_f32, 0.8944272_f32, 0.9486833_f32, 1_f32, 1.4142135_f32, 1.7320508_f32, 2_f32, 2.236068_f32, 2.4494898_f32, 2.6457512_f32, 2.828427_f32, 3_f32, 3.1622777_f32];
            coordinate_1 = (input_135471207135152[index(p) as usize] as f32);
            spline_index_2 = binary_search(density_function_Multiply_49_spline_coordinates_135471207138456, (coordinate_1 as f32));
            spline_result_f32_3 = advanced_hermite(density_function_Multiply_49_spline_coordinates_135471207138456, spline_values_0, density_function_Multiply_49_spline_derivatives_135471207138400, (coordinate_1 as f32), (spline_index_2 as i32));
            spline_result_f64_4 = (spline_result_f32_3 as f64);
            out[index(p)] = (spline_result_f64_4 * 0.8_f64);
        }
    }
    
    pub fn minecraft_new_surface_height_unprocessed_rivered_d5x1x5(out: &mut [f64; 25], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207193224: &[f64; 25], input_135471207191928: &[f64; 25], input_135471207189672: &[f64; 25], input_135471207178096: &[f64; 25], input_135471207173912: &[f64; 25], input_135471207165768: &[f64; 25], input_135471207157624: &[f64; 25], input_135471207156048: &[f64; 25], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut spline_values_0: [f32; 1];
        let mut coordinate_1: f32;
        let mut spline_index_2: i32;
        let mut spline_values_3: [f32; 2];
        let mut coordinate_4: f32;
        let mut spline_index_5: i32;
        let mut spline_values_6: [f32; 1];
        let mut coordinate_7: f32;
        let mut spline_index_8: i32;
        let mut spline_values_9: [f32; 2];
        let mut coordinate_10: f32;
        let mut spline_index_11: i32;
        let mut spline_values_12: [f32; 1];
        let mut coordinate_13: f32;
        let mut spline_index_14: i32;
        let mut spline_values_15: [f32; 3];
        let mut coordinate_16: f32;
        let mut spline_index_17: i32;
        let mut spline_values_18: [f32; 1];
        let mut coordinate_19: f32;
        let mut spline_index_20: i32;
        let mut spline_values_21: [f32; 4];
        let mut coordinate_22: f32;
        let mut spline_index_23: i32;
        let mut spline_values_24: [f32; 1];
        let mut coordinate_25: f32;
        let mut spline_index_26: i32;
        let mut spline_values_27: [f32; 5];
        let mut coordinate_28: f32;
        let mut spline_index_29: i32;
        let mut spline_values_30: [f32; 1];
        let mut coordinate_31: f32;
        let mut spline_index_32: i32;
        let mut spline_values_33: [f32; 6];
        let mut coordinate_34: f32;
        let mut spline_index_35: i32;
        let mut spline_values_36: [f32; 7];
        let mut coordinate_37: f32;
        let mut spline_index_38: i32;
        let mut spline_result_f32_39: f32;
        let mut spline_result_f64_40: f64;
        for p in iter_3d(5_i32, 1_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            spline_values_0 = [0_f32];
            coordinate_1 = (input_135471207193224[index(p) as usize] as f32);
            spline_index_2 = binary_search(minecraft_new_surface_height_unprocessed_rivered_spline_coordinates_135471207195368, (coordinate_1 as f32));
            spline_values_3 = [-0.9_f32, advanced_hermite(minecraft_new_surface_height_unprocessed_rivered_spline_coordinates_135471207195368, spline_values_0, minecraft_new_surface_height_unprocessed_rivered_spline_derivatives_135471207195312, (coordinate_1 as f32), (spline_index_2 as i32))];
            coordinate_4 = (input_135471207191928[index(p) as usize] as f32);
            spline_index_5 = binary_search(minecraft_new_surface_height_unprocessed_rivered_spline_coordinates_135471207193112, (coordinate_4 as f32));
            spline_values_6 = [0_f32];
            coordinate_7 = (input_135471207189672[index(p) as usize] as f32);
            spline_index_8 = binary_search(minecraft_new_surface_height_unprocessed_rivered_spline_coordinates_135471207191816, (coordinate_7 as f32));
            spline_values_9 = [advanced_hermite(minecraft_new_surface_height_unprocessed_rivered_spline_coordinates_135471207191816, spline_values_6, minecraft_new_surface_height_unprocessed_rivered_spline_derivatives_135471207191760, (coordinate_7 as f32), (spline_index_8 as i32)), advanced_hermite(minecraft_new_surface_height_unprocessed_rivered_spline_coordinates_135471207195368, spline_values_0, minecraft_new_surface_height_unprocessed_rivered_spline_derivatives_135471207195312, (coordinate_1 as f32), (spline_index_2 as i32))];
            coordinate_10 = (input_135471207191928[index(p) as usize] as f32);
            spline_index_11 = binary_search(minecraft_new_surface_height_unprocessed_rivered_spline_coordinates_135471207189560, (coordinate_10 as f32));
            spline_values_12 = [0_f32];
            coordinate_13 = (input_135471207178096[index(p) as usize] as f32);
            spline_index_14 = binary_search(minecraft_new_surface_height_unprocessed_rivered_spline_coordinates_135471207189280, (coordinate_13 as f32));
            spline_values_15 = [advanced_hermite(minecraft_new_surface_height_unprocessed_rivered_spline_coordinates_135471207189280, spline_values_12, minecraft_new_surface_height_unprocessed_rivered_spline_derivatives_135471207189224, (coordinate_13 as f32), (spline_index_14 as i32)), advanced_hermite(minecraft_new_surface_height_unprocessed_rivered_spline_coordinates_135471207191816, spline_values_6, minecraft_new_surface_height_unprocessed_rivered_spline_derivatives_135471207191760, (coordinate_7 as f32), (spline_index_8 as i32)), advanced_hermite(minecraft_new_surface_height_unprocessed_rivered_spline_coordinates_135471207195368, spline_values_0, minecraft_new_surface_height_unprocessed_rivered_spline_derivatives_135471207195312, (coordinate_1 as f32), (spline_index_2 as i32))];
            coordinate_16 = (input_135471207191928[index(p) as usize] as f32);
            spline_index_17 = binary_search(minecraft_new_surface_height_unprocessed_rivered_spline_coordinates_135471207177984, (coordinate_16 as f32));
            spline_values_18 = [0_f32];
            coordinate_19 = (input_135471207173912[index(p) as usize] as f32);
            spline_index_20 = binary_search(minecraft_new_surface_height_unprocessed_rivered_spline_coordinates_135471207177704, (coordinate_19 as f32));
            spline_values_21 = [advanced_hermite(minecraft_new_surface_height_unprocessed_rivered_spline_coordinates_135471207177704, spline_values_18, minecraft_new_surface_height_unprocessed_rivered_spline_derivatives_135471207177648, (coordinate_19 as f32), (spline_index_20 as i32)), advanced_hermite(minecraft_new_surface_height_unprocessed_rivered_spline_coordinates_135471207189280, spline_values_12, minecraft_new_surface_height_unprocessed_rivered_spline_derivatives_135471207189224, (coordinate_13 as f32), (spline_index_14 as i32)), advanced_hermite(minecraft_new_surface_height_unprocessed_rivered_spline_coordinates_135471207191816, spline_values_6, minecraft_new_surface_height_unprocessed_rivered_spline_derivatives_135471207191760, (coordinate_7 as f32), (spline_index_8 as i32)), advanced_hermite(minecraft_new_surface_height_unprocessed_rivered_spline_coordinates_135471207195368, spline_values_0, minecraft_new_surface_height_unprocessed_rivered_spline_derivatives_135471207195312, (coordinate_1 as f32), (spline_index_2 as i32))];
            coordinate_22 = (input_135471207191928[index(p) as usize] as f32);
            spline_index_23 = binary_search(minecraft_new_surface_height_unprocessed_rivered_spline_coordinates_135471207173800, (coordinate_22 as f32));
            spline_values_24 = [0_f32];
            coordinate_25 = (input_135471207165768[index(p) as usize] as f32);
            spline_index_26 = binary_search(minecraft_new_surface_height_unprocessed_rivered_spline_coordinates_135471207173520, (coordinate_25 as f32));
            spline_values_27 = [advanced_hermite(minecraft_new_surface_height_unprocessed_rivered_spline_coordinates_135471207173520, spline_values_24, minecraft_new_surface_height_unprocessed_rivered_spline_derivatives_135471207173464, (coordinate_25 as f32), (spline_index_26 as i32)), advanced_hermite(minecraft_new_surface_height_unprocessed_rivered_spline_coordinates_135471207177704, spline_values_18, minecraft_new_surface_height_unprocessed_rivered_spline_derivatives_135471207177648, (coordinate_19 as f32), (spline_index_20 as i32)), advanced_hermite(minecraft_new_surface_height_unprocessed_rivered_spline_coordinates_135471207189280, spline_values_12, minecraft_new_surface_height_unprocessed_rivered_spline_derivatives_135471207189224, (coordinate_13 as f32), (spline_index_14 as i32)), advanced_hermite(minecraft_new_surface_height_unprocessed_rivered_spline_coordinates_135471207191816, spline_values_6, minecraft_new_surface_height_unprocessed_rivered_spline_derivatives_135471207191760, (coordinate_7 as f32), (spline_index_8 as i32)), advanced_hermite(minecraft_new_surface_height_unprocessed_rivered_spline_coordinates_135471207195368, spline_values_0, minecraft_new_surface_height_unprocessed_rivered_spline_derivatives_135471207195312, (coordinate_1 as f32), (spline_index_2 as i32))];
            coordinate_28 = (input_135471207191928[index(p) as usize] as f32);
            spline_index_29 = binary_search(minecraft_new_surface_height_unprocessed_rivered_spline_coordinates_135471207165656, (coordinate_28 as f32));
            spline_values_30 = [0_f32];
            coordinate_31 = (input_135471207157624[index(p) as usize] as f32);
            spline_index_32 = binary_search(minecraft_new_surface_height_unprocessed_rivered_spline_coordinates_135471207165376, (coordinate_31 as f32));
            spline_values_33 = [advanced_hermite(minecraft_new_surface_height_unprocessed_rivered_spline_coordinates_135471207165376, spline_values_30, minecraft_new_surface_height_unprocessed_rivered_spline_derivatives_135471207165320, (coordinate_31 as f32), (spline_index_32 as i32)), advanced_hermite(minecraft_new_surface_height_unprocessed_rivered_spline_coordinates_135471207173520, spline_values_24, minecraft_new_surface_height_unprocessed_rivered_spline_derivatives_135471207173464, (coordinate_25 as f32), (spline_index_26 as i32)), advanced_hermite(minecraft_new_surface_height_unprocessed_rivered_spline_coordinates_135471207177704, spline_values_18, minecraft_new_surface_height_unprocessed_rivered_spline_derivatives_135471207177648, (coordinate_19 as f32), (spline_index_20 as i32)), advanced_hermite(minecraft_new_surface_height_unprocessed_rivered_spline_coordinates_135471207189280, spline_values_12, minecraft_new_surface_height_unprocessed_rivered_spline_derivatives_135471207189224, (coordinate_13 as f32), (spline_index_14 as i32)), advanced_hermite(minecraft_new_surface_height_unprocessed_rivered_spline_coordinates_135471207191816, spline_values_6, minecraft_new_surface_height_unprocessed_rivered_spline_derivatives_135471207191760, (coordinate_7 as f32), (spline_index_8 as i32)), advanced_hermite(minecraft_new_surface_height_unprocessed_rivered_spline_coordinates_135471207195368, spline_values_0, minecraft_new_surface_height_unprocessed_rivered_spline_derivatives_135471207195312, (coordinate_1 as f32), (spline_index_2 as i32))];
            coordinate_34 = (input_135471207191928[index(p) as usize] as f32);
            spline_index_35 = binary_search(minecraft_new_surface_height_unprocessed_rivered_spline_coordinates_135471207157512, (coordinate_34 as f32));
            spline_values_36 = [advanced_hermite(minecraft_new_surface_height_unprocessed_rivered_spline_coordinates_135471207193112, spline_values_3, minecraft_new_surface_height_unprocessed_rivered_spline_derivatives_135471207193056, (coordinate_4 as f32), (spline_index_5 as i32)), advanced_hermite(minecraft_new_surface_height_unprocessed_rivered_spline_coordinates_135471207189560, spline_values_9, minecraft_new_surface_height_unprocessed_rivered_spline_derivatives_135471207189504, (coordinate_10 as f32), (spline_index_11 as i32)), advanced_hermite(minecraft_new_surface_height_unprocessed_rivered_spline_coordinates_135471207177984, spline_values_15, minecraft_new_surface_height_unprocessed_rivered_spline_derivatives_135471207177928, (coordinate_16 as f32), (spline_index_17 as i32)), advanced_hermite(minecraft_new_surface_height_unprocessed_rivered_spline_coordinates_135471207177984, spline_values_15, minecraft_new_surface_height_unprocessed_rivered_spline_derivatives_135471207177928, (coordinate_16 as f32), (spline_index_17 as i32)), advanced_hermite(minecraft_new_surface_height_unprocessed_rivered_spline_coordinates_135471207173800, spline_values_21, minecraft_new_surface_height_unprocessed_rivered_spline_derivatives_135471207173744, (coordinate_22 as f32), (spline_index_23 as i32)), advanced_hermite(minecraft_new_surface_height_unprocessed_rivered_spline_coordinates_135471207165656, spline_values_27, minecraft_new_surface_height_unprocessed_rivered_spline_derivatives_135471207165600, (coordinate_28 as f32), (spline_index_29 as i32)), advanced_hermite(minecraft_new_surface_height_unprocessed_rivered_spline_coordinates_135471207157512, spline_values_33, minecraft_new_surface_height_unprocessed_rivered_spline_derivatives_135471207157456, (coordinate_34 as f32), (spline_index_35 as i32))];
            coordinate_37 = (input_135471207156048[index(p) as usize] as f32);
            spline_index_38 = binary_search(minecraft_new_surface_height_unprocessed_rivered_spline_coordinates_135471207157232, (coordinate_37 as f32));
            spline_result_f32_39 = advanced_hermite(minecraft_new_surface_height_unprocessed_rivered_spline_coordinates_135471207157232, spline_values_36, minecraft_new_surface_height_unprocessed_rivered_spline_derivatives_135471207157176, (coordinate_37 as f32), (spline_index_38 as i32));
            spline_result_f64_40 = (spline_result_f32_39 as f64);
            out[index(p)] = spline_result_f64_40;
        }
    }
    
    pub fn minecraft_gravel_19_d5x1x5(out: &mut [f64; 25], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, perm_table_minecraft_gravel_0_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__5: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__5: &PerlinNoiseSampler, host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut n: f64;
        for p in iter_3d(5_i32, 1_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * Vec3::new(0.2_f64, 0_f64, 0.2_f64)) + (pos3 * Vec3::new(0.8_f64, 0_f64, 0.8_f64)));
            n = 0_f64;
            n = (n + (perlin((rpos3 * 0.00390625_f64), perm_table_minecraft_gravel_0_octave__8) * 0.5333333333333333_f64));
            n = (n + (perlin((rpos3 * 0.003977058157099698_f64), perm_table_minecraft_gravel_1_octave__8) * 0.5333333333333333_f64));
            n = (n + (perlin((rpos3 * 0.0078125_f64), perm_table_minecraft_gravel_0_octave__7) * 0.26666666666666666_f64));
            n = (n + (perlin((rpos3 * 0.007954116314199396_f64), perm_table_minecraft_gravel_1_octave__7) * 0.26666666666666666_f64));
            n = (n + (perlin((rpos3 * 0.015625_f64), perm_table_minecraft_gravel_0_octave__6) * 0.13333333333333333_f64));
            n = (n + (perlin((rpos3 * 0.015908232628398793_f64), perm_table_minecraft_gravel_1_octave__6) * 0.13333333333333333_f64));
            n = (n + (perlin((rpos3 * 0.03125_f64), perm_table_minecraft_gravel_0_octave__5) * 0.06666666666666667_f64));
            n = (n + (perlin((rpos3 * 0.031816465256797585_f64), perm_table_minecraft_gravel_1_octave__5) * 0.06666666666666667_f64));
            out[index(p)] = (n * 1.3333333333333333_f64);
        }
    }
    
    pub fn minecraft_realism_hills_jagged_original_d5x1x5(out: &mut [f64; 25], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207180296: &[f64; 25], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        for p in iter_3d(5_i32, 1_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            out[index(p)] = input_135471207180296[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize];
        }
    }
    
    pub fn minecraft_realism_extreme_mountains_factor_original_d5x263x5(out: &mut [f64; 6575], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207163968: &[f64; 25], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        for p in iter_3d(5_i32, 263_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            out[index(p)] = input_135471207163968[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize];
        }
    }
    
    pub fn minecraft_cave_layer_69_d5x263x5(out: &mut [f64; 6575], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, perm_table_minecraft_cave_layer_0_octave__8: &PerlinNoiseSampler, perm_table_minecraft_cave_layer_1_octave__8: &PerlinNoiseSampler, host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut n: f64;
        for p in iter_3d(5_i32, 263_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * Vec3::new(4_f64, 4_f64, 4_f64)) + (pos3 * Vec3::new(16_f64, 32_f64, 16_f64)));
            n = 0_f64;
            n = (n + (perlin((rpos3 * 0.00390625_f64), perm_table_minecraft_cave_layer_0_octave__8) * 1_f64));
            n = (n + (perlin((rpos3 * 0.003977058157099698_f64), perm_table_minecraft_cave_layer_1_octave__8) * 1_f64));
            out[index(p)] = (n * 0.8333333333333333_f64);
        }
    }
    
    pub fn minecraft_realism_extreme_hills_slope_height_d5x263x5(out: &mut [f64; 6575], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207194440: &[f64; 25], input_135471207139744: &[f64; 6575], input_135471207134416: &[f64; 6575], input_135471207128800: &[f64; 6575], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut spline_values_0: [f32; 1];
        let mut coordinate_1: f32;
        let mut spline_index_2: i32;
        let mut spline_values_3: [f32; 1];
        let mut coordinate_4: f32;
        let mut spline_index_5: i32;
        let mut spline_values_6: [f32; 2];
        let mut coordinate_7: f32;
        let mut spline_index_8: i32;
        let mut spline_result_f32_9: f32;
        let mut spline_result_f64_10: f64;
        let mut spline_values_11: [f32; 1];
        let mut coordinate_12: f32;
        let mut spline_index_13: i32;
        let mut spline_values_14: [f32; 1];
        let mut coordinate_15: f32;
        let mut spline_index_16: i32;
        let mut spline_values_17: [f32; 2];
        let mut coordinate_18: f32;
        let mut spline_index_19: i32;
        let mut spline_result_f32_20: f32;
        let mut spline_result_f64_21: f64;
        for p in iter_3d(5_i32, 263_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            spline_values_0 = [0_f32];
            coordinate_1 = (input_135471207139744[index(p) as usize] as f32);
            spline_index_2 = binary_search(minecraft_realism_extreme_hills_slope_height_spline_coordinates_135471207130624, (coordinate_1 as f32));
            spline_values_3 = [0_f32];
            coordinate_4 = (input_135471207134416[index(p) as usize] as f32);
            spline_index_5 = binary_search(minecraft_realism_extreme_hills_slope_height_spline_coordinates_135471207130288, (coordinate_4 as f32));
            spline_values_6 = [advanced_hermite(minecraft_realism_extreme_hills_slope_height_spline_coordinates_135471207130624, spline_values_0, minecraft_realism_extreme_hills_slope_height_spline_derivatives_135471207130568, (coordinate_1 as f32), (spline_index_2 as i32)), advanced_hermite(minecraft_realism_extreme_hills_slope_height_spline_coordinates_135471207130288, spline_values_3, minecraft_realism_extreme_hills_slope_height_spline_derivatives_135471207130232, (coordinate_4 as f32), (spline_index_5 as i32))];
            coordinate_7 = (input_135471207139744[index(p) as usize] as f32);
            spline_index_8 = binary_search(minecraft_realism_extreme_hills_slope_height_spline_coordinates_135471207129952, (coordinate_7 as f32));
            spline_result_f32_9 = advanced_hermite(minecraft_realism_extreme_hills_slope_height_spline_coordinates_135471207129952, spline_values_6, minecraft_realism_extreme_hills_slope_height_spline_derivatives_135471207129896, (coordinate_7 as f32), (spline_index_8 as i32));
            spline_result_f64_10 = (spline_result_f32_9 as f64);
            spline_values_11 = [0_f32];
            coordinate_12 = (input_135471207128800[index(p) as usize] as f32);
            spline_index_13 = binary_search(minecraft_realism_extreme_hills_slope_height_spline_coordinates_135471207129560, (coordinate_12 as f32));
            spline_values_14 = [0_f32];
            coordinate_15 = (input_135471207128800[index(p) as usize] as f32);
            spline_index_16 = binary_search(minecraft_realism_extreme_hills_slope_height_spline_coordinates_135471207128688, (coordinate_15 as f32));
            spline_values_17 = [advanced_hermite(minecraft_realism_extreme_hills_slope_height_spline_coordinates_135471207129560, spline_values_11, minecraft_realism_extreme_hills_slope_height_spline_derivatives_135471207129504, (coordinate_12 as f32), (spline_index_13 as i32)), advanced_hermite(minecraft_realism_extreme_hills_slope_height_spline_coordinates_135471207128688, spline_values_14, minecraft_realism_extreme_hills_slope_height_spline_derivatives_135471207128632, (coordinate_15 as f32), (spline_index_16 as i32))];
            coordinate_18 = (input_135471207139744[index(p) as usize] as f32);
            spline_index_19 = binary_search(minecraft_realism_extreme_hills_slope_height_spline_coordinates_135471207128408, (coordinate_18 as f32));
            spline_result_f32_20 = advanced_hermite(minecraft_realism_extreme_hills_slope_height_spline_coordinates_135471207128408, spline_values_17, minecraft_realism_extreme_hills_slope_height_spline_derivatives_135471207128352, (coordinate_18 as f32), (spline_index_19 as i32));
            spline_result_f64_21 = (spline_result_f32_20 as f64);
            out[index(p)] = ((0_f64 + ((input_135471207194440[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * 0.4_f64) + -0.2_f64)) + ((spline_result_f64_10 * 0.3_f64) + spline_result_f64_21));
        }
    }
    
    pub fn minecraft_new_surface_combined_processed_base_d5x263x5(out: &mut [f64; 6575], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207155680: &[f64; 25], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut var_0: f64;
        for p in iter_3d(5_i32, 263_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            var_0 = clamp(((rpos3.y - -64_f64) / 2096_f64), 0_f64, 1_f64);
            out[index(p)] = (((var_0 * -2_f64) + 1_f64) + input_135471207155680[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize]);
        }
    }
    
    pub fn minecraft_caves_new_medium_d5x263x5(out: &mut [f64; 6575], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207100120: &[f64; 6575], input_135471207097024: &[f64; 6575], input_135471207189928: &[f64; 25], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut spline_values_0: [f32; 81];
        let mut coordinate_1: f32;
        let mut spline_index_2: i32;
        let mut spline_result_f32_3: f32;
        let mut spline_result_f64_4: f64;
        for p in iter_3d(5_i32, 263_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            spline_values_0 = [1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32, -1_f32, 1_f32];
            coordinate_1 = (input_135471207100120[index(p) as usize] as f32);
            spline_index_2 = binary_search(minecraft_caves_new_medium_spline_coordinates_135471207097840, (coordinate_1 as f32));
            spline_result_f32_3 = advanced_hermite(minecraft_caves_new_medium_spline_coordinates_135471207097840, spline_values_0, minecraft_caves_new_medium_spline_derivatives_135471207097784, (coordinate_1 as f32), (spline_index_2 as i32));
            spline_result_f64_4 = (spline_result_f32_3 as f64);
            out[index(p)] = (spline_result_f64_4 + ((input_135471207097024[index(p) as usize] * 1.5_f64) + (input_135471207189928[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * 0.5_f64)));
        }
    }
    
    pub fn minecraft_cave_layer_22_d5x1x5(out: &mut [f64; 25], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, perm_table_minecraft_cave_layer_0_octave__8: &PerlinNoiseSampler, perm_table_minecraft_cave_layer_1_octave__8: &PerlinNoiseSampler, host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut n: f64;
        for p in iter_3d(5_i32, 1_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * Vec3::new(0.3_f64, 0_f64, 0.3_f64)) + (pos3 * Vec3::new(1.2_f64, 0_f64, 1.2_f64)));
            n = 0_f64;
            n = (n + (perlin((rpos3 * 0.00390625_f64), perm_table_minecraft_cave_layer_0_octave__8) * 1_f64));
            n = (n + (perlin((rpos3 * 0.003977058157099698_f64), perm_table_minecraft_cave_layer_1_octave__8) * 1_f64));
            out[index(p)] = (n * 0.8333333333333333_f64);
        }
    }
    
    pub fn density_function_ShiftedNoise_58_d5x263x5(out: &mut [f64; 6575], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, perm_table_minecraft_gravel_0_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__5: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__5: &PerlinNoiseSampler, host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        for p in iter_3d(5_i32, 263_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            out[index(p)] = density_function_ShiftedNoise_58_d5x263x5_minecraft_gravel_shifted_59(((rpos3 * Vec3::new(0.11_f64, 0_f64, 0.11_f64)) + Vec3::new(0_f64, 0_f64, 1_f64)), perm_table_minecraft_gravel_0_octave__8, perm_table_minecraft_gravel_1_octave__8, perm_table_minecraft_gravel_0_octave__7, perm_table_minecraft_gravel_1_octave__7, perm_table_minecraft_gravel_0_octave__6, perm_table_minecraft_gravel_1_octave__6, perm_table_minecraft_gravel_0_octave__5, perm_table_minecraft_gravel_1_octave__5, host_consts);
        }
    }
    
    pub fn minecraft_jagged_39_d5x1x5(out: &mut [f64; 25], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, perm_table_minecraft_jagged_0_octave__16: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__16: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__15: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__15: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__14: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__14: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__13: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__13: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__12: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__12: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__11: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__11: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__10: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__10: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__9: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__9: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__8: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__8: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__7: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__7: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__6: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__6: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__5: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__5: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__4: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__4: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__3: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__3: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__2: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__2: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__1: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__1: &PerlinNoiseSampler, host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut n: f64;
        for p in iter_3d(5_i32, 1_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * Vec3::new(2_f64, 0_f64, 2_f64)) + (pos3 * Vec3::new(8_f64, 0_f64, 8_f64)));
            n = 0_f64;
            n = (n + (perlin((rpos3 * 0.0000152587890625_f64), perm_table_minecraft_jagged_0_octave__16) * 0.5000076295109483_f64));
            n = (n + (perlin((rpos3 * 0.000015535383426170696_f64), perm_table_minecraft_jagged_1_octave__16) * 0.5000076295109483_f64));
            n = (n + (perlin((rpos3 * 0.000030517578125_f64), perm_table_minecraft_jagged_0_octave__15) * 0.2500038147554742_f64));
            n = (n + (perlin((rpos3 * 0.00003107076685234139_f64), perm_table_minecraft_jagged_1_octave__15) * 0.2500038147554742_f64));
            n = (n + (perlin((rpos3 * 0.00006103515625_f64), perm_table_minecraft_jagged_0_octave__14) * 0.1250019073777371_f64));
            n = (n + (perlin((rpos3 * 0.00006214153370468278_f64), perm_table_minecraft_jagged_1_octave__14) * 0.1250019073777371_f64));
            n = (n + (perlin((rpos3 * 0.0001220703125_f64), perm_table_minecraft_jagged_0_octave__13) * 0.06250095368886854_f64));
            n = (n + (perlin((rpos3 * 0.00012428306740936557_f64), perm_table_minecraft_jagged_1_octave__13) * 0.06250095368886854_f64));
            n = (n + (perlin((rpos3 * 0.000244140625_f64), perm_table_minecraft_jagged_0_octave__12) * 0.03125047684443427_f64));
            n = (n + (perlin((rpos3 * 0.00024856613481873114_f64), perm_table_minecraft_jagged_1_octave__12) * 0.03125047684443427_f64));
            n = (n + (perlin((rpos3 * 0.00048828125_f64), perm_table_minecraft_jagged_0_octave__11) * 0.015625238422217136_f64));
            n = (n + (perlin((rpos3 * 0.0004971322696374623_f64), perm_table_minecraft_jagged_1_octave__11) * 0.015625238422217136_f64));
            n = (n + (perlin((rpos3 * 0.0009765625_f64), perm_table_minecraft_jagged_0_octave__10) * 0.007812619211108568_f64));
            n = (n + (perlin((rpos3 * 0.0009942645392749245_f64), perm_table_minecraft_jagged_1_octave__10) * 0.007812619211108568_f64));
            n = (n + (perlin((rpos3 * 0.001953125_f64), perm_table_minecraft_jagged_0_octave__9) * 0.003906309605554284_f64));
            n = (n + (perlin((rpos3 * 0.001988529078549849_f64), perm_table_minecraft_jagged_1_octave__9) * 0.003906309605554284_f64));
            n = (n + (perlin((rpos3 * 0.00390625_f64), perm_table_minecraft_jagged_0_octave__8) * 0.001953154802777142_f64));
            n = (n + (perlin((rpos3 * 0.003977058157099698_f64), perm_table_minecraft_jagged_1_octave__8) * 0.001953154802777142_f64));
            n = (n + (perlin((rpos3 * 0.0078125_f64), perm_table_minecraft_jagged_0_octave__7) * 0.000976577401388571_f64));
            n = (n + (perlin((rpos3 * 0.007954116314199396_f64), perm_table_minecraft_jagged_1_octave__7) * 0.000976577401388571_f64));
            n = (n + (perlin((rpos3 * 0.015625_f64), perm_table_minecraft_jagged_0_octave__6) * 0.0004882887006942855_f64));
            n = (n + (perlin((rpos3 * 0.015908232628398793_f64), perm_table_minecraft_jagged_1_octave__6) * 0.0004882887006942855_f64));
            n = (n + (perlin((rpos3 * 0.03125_f64), perm_table_minecraft_jagged_0_octave__5) * 0.00024414435034714275_f64));
            n = (n + (perlin((rpos3 * 0.031816465256797585_f64), perm_table_minecraft_jagged_1_octave__5) * 0.00024414435034714275_f64));
            n = (n + (perlin((rpos3 * 0.0625_f64), perm_table_minecraft_jagged_0_octave__4) * 0.00012207217517357137_f64));
            n = (n + (perlin((rpos3 * 0.06363293051359517_f64), perm_table_minecraft_jagged_1_octave__4) * 0.00012207217517357137_f64));
            n = (n + (perlin((rpos3 * 0.125_f64), perm_table_minecraft_jagged_0_octave__3) * 0.00006103608758678569_f64));
            n = (n + (perlin((rpos3 * 0.12726586102719034_f64), perm_table_minecraft_jagged_1_octave__3) * 0.00006103608758678569_f64));
            n = (n + (perlin((rpos3 * 0.25_f64), perm_table_minecraft_jagged_0_octave__2) * 0.000030518043793392844_f64));
            n = (n + (perlin((rpos3 * 0.2545317220543807_f64), perm_table_minecraft_jagged_1_octave__2) * 0.000030518043793392844_f64));
            n = (n + (perlin((rpos3 * 0.5_f64), perm_table_minecraft_jagged_0_octave__1) * 0.000015259021896696422_f64));
            n = (n + (perlin((rpos3 * 0.5090634441087614_f64), perm_table_minecraft_jagged_1_octave__1) * 0.000015259021896696422_f64));
            out[index(p)] = (n * 1.568627450980392_f64);
        }
    }
    
    pub fn minecraft_realism_extreme_hills_factor_divergence_squared_d5x263x5(out: &mut [f64; 6575], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207136680: &[f64; 6575], input_135471207187312: &[f64; 25], input_135471207135352: &[f64; 6575], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut var_0: f64;
        let mut var_1: f64;
        for p in iter_3d(5_i32, 263_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            var_0 = (input_135471207136680[index(p) as usize] + (input_135471207187312[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * -1_f64));
            var_1 = (input_135471207135352[index(p) as usize] + (input_135471207187312[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * -1_f64));
            out[index(p)] = (((var_0 * var_0) + (var_1 * var_1)) * 10000_f64);
        }
    }
    
    pub fn density_function_Multiply_32_d5x1x5(out: &mut [f64; 25], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207162080: &[f64; 25], input_135471207163968: &[f64; 25], input_135471207160752: &[f64; 25], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut var_0: f64;
        let mut var_1: f64;
        for p in iter_3d(5_i32, 1_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            var_0 = (input_135471207162080[index(p) as usize] + (input_135471207163968[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * -1_f64));
            var_1 = (input_135471207160752[index(p) as usize] + (input_135471207163968[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * -1_f64));
            out[index(p)] = ((((var_0 * var_0) + (var_1 * var_1)) * 5000_f64) * 0.2_f64);
        }
    }
    
    pub fn minecraft_realism_hills_slope_height_d5x263x5(out: &mut [f64; 6575], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207188352: &[f64; 25], input_135471207139744: &[f64; 6575], input_135471207134416: &[f64; 6575], input_135471207133152: &[f64; 6575], input_135471207132000: &[f64; 6575], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut spline_values_0: [f32; 1];
        let mut coordinate_1: f32;
        let mut spline_index_2: i32;
        let mut spline_values_3: [f32; 1];
        let mut coordinate_4: f32;
        let mut spline_index_5: i32;
        let mut spline_values_6: [f32; 2];
        let mut coordinate_7: f32;
        let mut spline_index_8: i32;
        let mut spline_result_f32_9: f32;
        let mut spline_result_f64_10: f64;
        let mut spline_values_11: [f32; 1];
        let mut coordinate_12: f32;
        let mut spline_index_13: i32;
        let mut spline_values_14: [f32; 1];
        let mut coordinate_15: f32;
        let mut spline_index_16: i32;
        let mut spline_values_17: [f32; 2];
        let mut coordinate_18: f32;
        let mut spline_index_19: i32;
        let mut spline_result_f32_20: f32;
        let mut spline_result_f64_21: f64;
        for p in iter_3d(5_i32, 263_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            spline_values_0 = [0_f32];
            coordinate_1 = (input_135471207139744[index(p) as usize] as f32);
            spline_index_2 = binary_search(minecraft_realism_hills_slope_height_spline_coordinates_135471207140504, (coordinate_1 as f32));
            spline_values_3 = [0_f32];
            coordinate_4 = (input_135471207134416[index(p) as usize] as f32);
            spline_index_5 = binary_search(minecraft_realism_hills_slope_height_spline_coordinates_135471207139632, (coordinate_4 as f32));
            spline_values_6 = [advanced_hermite(minecraft_realism_hills_slope_height_spline_coordinates_135471207140504, spline_values_0, minecraft_realism_hills_slope_height_spline_derivatives_135471207140448, (coordinate_1 as f32), (spline_index_2 as i32)), advanced_hermite(minecraft_realism_hills_slope_height_spline_coordinates_135471207139632, spline_values_3, minecraft_realism_hills_slope_height_spline_derivatives_135471207139576, (coordinate_4 as f32), (spline_index_5 as i32))];
            coordinate_7 = (input_135471207139744[index(p) as usize] as f32);
            spline_index_8 = binary_search(minecraft_realism_hills_slope_height_spline_coordinates_135471207134304, (coordinate_7 as f32));
            spline_result_f32_9 = advanced_hermite(minecraft_realism_hills_slope_height_spline_coordinates_135471207134304, spline_values_6, minecraft_realism_hills_slope_height_spline_derivatives_135471207134248, (coordinate_7 as f32), (spline_index_8 as i32));
            spline_result_f64_10 = (spline_result_f32_9 as f64);
            spline_values_11 = [0_f32];
            coordinate_12 = (input_135471207133152[index(p) as usize] as f32);
            spline_index_13 = binary_search(minecraft_realism_hills_slope_height_spline_coordinates_135471207133912, (coordinate_12 as f32));
            spline_values_14 = [0_f32];
            coordinate_15 = (input_135471207133152[index(p) as usize] as f32);
            spline_index_16 = binary_search(minecraft_realism_hills_slope_height_spline_coordinates_135471207133040, (coordinate_15 as f32));
            spline_values_17 = [advanced_hermite(minecraft_realism_hills_slope_height_spline_coordinates_135471207133912, spline_values_11, minecraft_realism_hills_slope_height_spline_derivatives_135471207133856, (coordinate_12 as f32), (spline_index_13 as i32)), advanced_hermite(minecraft_realism_hills_slope_height_spline_coordinates_135471207133040, spline_values_14, minecraft_realism_hills_slope_height_spline_derivatives_135471207132984, (coordinate_15 as f32), (spline_index_16 as i32))];
            coordinate_18 = (input_135471207132000[index(p) as usize] as f32);
            spline_index_19 = binary_search(minecraft_realism_hills_slope_height_spline_coordinates_135471207132760, (coordinate_18 as f32));
            spline_result_f32_20 = advanced_hermite(minecraft_realism_hills_slope_height_spline_coordinates_135471207132760, spline_values_17, minecraft_realism_hills_slope_height_spline_derivatives_135471207132704, (coordinate_18 as f32), (spline_index_19 as i32));
            spline_result_f64_21 = (spline_result_f32_20 as f64);
            out[index(p)] = ((0_f64 + ((input_135471207188352[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * 0.25_f64) + -0.5_f64)) + ((spline_result_f64_10 * 0.2_f64) + spline_result_f64_21));
        }
    }
    
    pub fn minecraft_new_surface_height_processed_base_d5x263x5(out: &mut [f64; 6575], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207155680: &[f64; 25], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        for p in iter_3d(5_i32, 263_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            out[index(p)] = input_135471207155680[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize];
        }
    }
    
    pub fn minecraft_new_surface_height_unprocessed_hollowed_d5x1x5(out: &mut [f64; 25], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207155680: &[f64; 25], input_135471207146464: &[f64; 25], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut spline_values_0: [f32; 1];
        let mut coordinate_1: f32;
        let mut spline_index_2: i32;
        let mut spline_values_3: [f32; 1];
        let mut coordinate_4: f32;
        let mut spline_index_5: i32;
        let mut spline_values_6: [f32; 2];
        let mut coordinate_7: f32;
        let mut spline_index_8: i32;
        let mut spline_result_f32_9: f32;
        let mut spline_result_f64_10: f64;
        for p in iter_3d(5_i32, 1_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            spline_values_0 = [0_f32];
            coordinate_1 = (input_135471207155680[index(p) as usize] as f32);
            spline_index_2 = binary_search(minecraft_new_surface_height_unprocessed_hollowed_spline_coordinates_135471207195928, (coordinate_1 as f32));
            spline_values_3 = [0_f32];
            coordinate_4 = (input_135471207146464[index(p) as usize] as f32);
            spline_index_5 = binary_search(minecraft_new_surface_height_unprocessed_hollowed_spline_coordinates_135471207155568, (coordinate_4 as f32));
            spline_values_6 = [advanced_hermite(minecraft_new_surface_height_unprocessed_hollowed_spline_coordinates_135471207195928, spline_values_0, minecraft_new_surface_height_unprocessed_hollowed_spline_derivatives_135471207195872, (coordinate_1 as f32), (spline_index_2 as i32)), advanced_hermite(minecraft_new_surface_height_unprocessed_hollowed_spline_coordinates_135471207155568, spline_values_3, minecraft_new_surface_height_unprocessed_hollowed_spline_derivatives_135471207155512, (coordinate_4 as f32), (spline_index_5 as i32))];
            coordinate_7 = (input_135471207155680[index(p) as usize] as f32);
            spline_index_8 = binary_search(minecraft_new_surface_height_unprocessed_hollowed_spline_coordinates_135471207146352, (coordinate_7 as f32));
            spline_result_f32_9 = advanced_hermite(minecraft_new_surface_height_unprocessed_hollowed_spline_coordinates_135471207146352, spline_values_6, minecraft_new_surface_height_unprocessed_hollowed_spline_derivatives_135471207146296, (coordinate_7 as f32), (spline_index_8 as i32));
            spline_result_f64_10 = (spline_result_f32_9 as f64);
            out[index(p)] = spline_result_f64_10;
        }
    }
    
    pub fn minecraft_new_combination_unfixed_3_d5x263x5(out: &mut [f64; 6575], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207145616: &[f64; 6575], input_135471207144688: &[f64; 6575], input_135471207112608: &[f64; 6575], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut spline_values_0: [f32; 1];
        let mut coordinate_1: f32;
        let mut spline_index_2: i32;
        let mut spline_values_3: [f32; 1];
        let mut coordinate_4: f32;
        let mut spline_index_5: i32;
        let mut spline_values_6: [f32; 2];
        let mut coordinate_7: f32;
        let mut spline_index_8: i32;
        let mut spline_result_f32_9: f32;
        let mut spline_result_f64_10: f64;
        for p in iter_3d(5_i32, 263_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            spline_values_0 = [0_f32];
            coordinate_1 = (input_135471207145616[index(p) as usize] as f32);
            spline_index_2 = binary_search(minecraft_new_combination_unfixed_3_spline_coordinates_135471207196880, (coordinate_1 as f32));
            spline_values_3 = [0_f32];
            coordinate_4 = (input_135471207144688[index(p) as usize] as f32);
            spline_index_5 = binary_search(minecraft_new_combination_unfixed_3_spline_coordinates_135471207145504, (coordinate_4 as f32));
            spline_values_6 = [advanced_hermite(minecraft_new_combination_unfixed_3_spline_coordinates_135471207196880, spline_values_0, minecraft_new_combination_unfixed_3_spline_derivatives_135471207196824, (coordinate_1 as f32), (spline_index_2 as i32)), advanced_hermite(minecraft_new_combination_unfixed_3_spline_coordinates_135471207145504, spline_values_3, minecraft_new_combination_unfixed_3_spline_derivatives_135471207145448, (coordinate_4 as f32), (spline_index_5 as i32))];
            coordinate_7 = (input_135471207112608[index(p) as usize] as f32);
            spline_index_8 = binary_search(minecraft_new_combination_unfixed_3_spline_coordinates_135471207144576, (coordinate_7 as f32));
            spline_result_f32_9 = advanced_hermite(minecraft_new_combination_unfixed_3_spline_coordinates_135471207144576, spline_values_6, minecraft_new_combination_unfixed_3_spline_derivatives_135471207144520, (coordinate_7 as f32), (spline_index_8 as i32));
            spline_result_f64_10 = (spline_result_f32_9 as f64);
            out[index(p)] = spline_result_f64_10;
        }
    }
    
    pub fn minecraft_cave_layer_1_d5x1x5(out: &mut [f64; 25], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, perm_table_minecraft_cave_layer_0_octave__8: &PerlinNoiseSampler, perm_table_minecraft_cave_layer_1_octave__8: &PerlinNoiseSampler, host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut n: f64;
        for p in iter_3d(5_i32, 1_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * Vec3::new(0.2_f64, 0_f64, 0.2_f64)) + (pos3 * Vec3::new(0.8_f64, 0_f64, 0.8_f64)));
            n = 0_f64;
            n = (n + (perlin((rpos3 * 0.00390625_f64), perm_table_minecraft_cave_layer_0_octave__8) * 1_f64));
            n = (n + (perlin((rpos3 * 0.003977058157099698_f64), perm_table_minecraft_cave_layer_1_octave__8) * 1_f64));
            out[index(p)] = (n * 0.8333333333333333_f64);
        }
    }
    
    pub fn density_function_Add_0_d5x1x5(out: &mut [f64; 25], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207194440: &[f64; 25], input_135471207193960: &[f64; 25], input_135471207193480: &[f64; 25], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        for p in iter_3d(5_i32, 1_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            out[index(p)] = (((input_135471207194440[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * 0.09_f64) + -0.915_f64) + (((input_135471207193960[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * 0.05_f64) + 0_f64) + ((input_135471207193480[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * 0.015_f64) + 0_f64)));
        }
    }
    
    pub fn minecraft_cave_layer_74_d5x263x5(out: &mut [f64; 6575], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, perm_table_minecraft_cave_layer_0_octave__8: &PerlinNoiseSampler, perm_table_minecraft_cave_layer_1_octave__8: &PerlinNoiseSampler, host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut n: f64;
        for p in iter_3d(5_i32, 263_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * Vec3::new(2.5_f64, 1_f64, 2.5_f64)) + (pos3 * Vec3::new(10_f64, 8_f64, 10_f64)));
            n = 0_f64;
            n = (n + (perlin((rpos3 * 0.00390625_f64), perm_table_minecraft_cave_layer_0_octave__8) * 1_f64));
            n = (n + (perlin((rpos3 * 0.003977058157099698_f64), perm_table_minecraft_cave_layer_1_octave__8) * 1_f64));
            out[index(p)] = (n * 0.8333333333333333_f64);
        }
    }
    
    pub fn minecraft_new_combination_unfixed_new_d5x263x5(out: &mut [f64; 6575], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207112240: &[f64; 6575], input_135471207110304: &[f64; 6575], input_135471207109432: &[f64; 6575], input_135471207107744: &[f64; 6575], input_135471207106448: &[f64; 6575], input_135471207105152: &[f64; 6575], input_135471207103576: &[f64; 6575], input_135471207144688: &[f64; 6575], input_135471207102032: &[f64; 6575], input_135471207098792: &[f64; 6575], input_135471207096712: &[f64; 6575], input_135471207094632: &[f64; 6575], input_135471207092776: &[f64; 6575], input_135471207145616: &[f64; 6575], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut spline_values_0: [f32; 1];
        let mut coordinate_1: f32;
        let mut spline_index_2: i32;
        let mut spline_values_3: [f32; 1];
        let mut coordinate_4: f32;
        let mut spline_index_5: i32;
        let mut spline_values_6: [f32; 2];
        let mut coordinate_7: f32;
        let mut spline_index_8: i32;
        let mut spline_result_f32_9: f32;
        let mut spline_result_f64_10: f64;
        let mut spline_values_11: [f32; 1];
        let mut coordinate_12: f32;
        let mut spline_index_13: i32;
        let mut spline_values_14: [f32; 1];
        let mut coordinate_15: f32;
        let mut spline_index_16: i32;
        let mut spline_values_17: [f32; 2];
        let mut coordinate_18: f32;
        let mut spline_index_19: i32;
        let mut spline_values_20: [f32; 1];
        let mut coordinate_21: f32;
        let mut spline_index_22: i32;
        let mut spline_values_23: [f32; 2];
        let mut coordinate_24: f32;
        let mut spline_index_25: i32;
        let mut spline_values_26: [f32; 1];
        let mut coordinate_27: f32;
        let mut spline_index_28: i32;
        let mut spline_values_29: [f32; 2];
        let mut coordinate_30: f32;
        let mut spline_index_31: i32;
        let mut spline_values_32: [f32; 1];
        let mut coordinate_33: f32;
        let mut spline_index_34: i32;
        let mut spline_values_35: [f32; 4];
        let mut coordinate_36: f32;
        let mut spline_index_37: i32;
        let mut spline_values_38: [f32; 1];
        let mut coordinate_39: f32;
        let mut spline_index_40: i32;
        let mut spline_values_41: [f32; 2];
        let mut coordinate_42: f32;
        let mut spline_index_43: i32;
        let mut spline_values_44: [f32; 1];
        let mut coordinate_45: f32;
        let mut spline_index_46: i32;
        let mut spline_values_47: [f32; 2];
        let mut coordinate_48: f32;
        let mut spline_index_49: i32;
        let mut spline_values_50: [f32; 1];
        let mut coordinate_51: f32;
        let mut spline_index_52: i32;
        let mut spline_values_53: [f32; 2];
        let mut coordinate_54: f32;
        let mut spline_index_55: i32;
        let mut spline_values_56: [f32; 4];
        let mut coordinate_57: f32;
        let mut spline_index_58: i32;
        let mut spline_values_59: [f32; 2];
        let mut coordinate_60: f32;
        let mut spline_index_61: i32;
        let mut spline_values_62: [f32; 2];
        let mut coordinate_63: f32;
        let mut spline_index_64: i32;
        let mut spline_result_f32_65: f32;
        let mut spline_result_f64_66: f64;
        for p in iter_3d(5_i32, 263_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            spline_values_0 = [0_f32];
            coordinate_1 = (input_135471207112240[index(p) as usize] as f32);
            spline_index_2 = binary_search(minecraft_new_combination_unfixed_new_spline_coordinates_135471207197440, (coordinate_1 as f32));
            spline_values_3 = [0_f32];
            coordinate_4 = (input_135471207110304[index(p) as usize] as f32);
            spline_index_5 = binary_search(minecraft_new_combination_unfixed_new_spline_coordinates_135471207112128, (coordinate_4 as f32));
            spline_values_6 = [advanced_hermite(minecraft_new_combination_unfixed_new_spline_coordinates_135471207197440, spline_values_0, minecraft_new_combination_unfixed_new_spline_derivatives_135471207197384, (coordinate_1 as f32), (spline_index_2 as i32)), advanced_hermite(minecraft_new_combination_unfixed_new_spline_coordinates_135471207112128, spline_values_3, minecraft_new_combination_unfixed_new_spline_derivatives_135471207112072, (coordinate_4 as f32), (spline_index_5 as i32))];
            coordinate_7 = (input_135471207109432[index(p) as usize] as f32);
            spline_index_8 = binary_search(minecraft_new_combination_unfixed_new_spline_coordinates_135471207110192, (coordinate_7 as f32));
            spline_result_f32_9 = advanced_hermite(minecraft_new_combination_unfixed_new_spline_coordinates_135471207110192, spline_values_6, minecraft_new_combination_unfixed_new_spline_derivatives_135471207110136, (coordinate_7 as f32), (spline_index_8 as i32));
            spline_result_f64_10 = (spline_result_f32_9 as f64);
            spline_values_11 = [0_f32];
            coordinate_12 = (input_135471207110304[index(p) as usize] as f32);
            spline_index_13 = binary_search(minecraft_new_combination_unfixed_new_spline_coordinates_135471207109208, (coordinate_12 as f32));
            spline_values_14 = [0_f32];
            coordinate_15 = (input_135471207107744[index(p) as usize] as f32);
            spline_index_16 = binary_search(minecraft_new_combination_unfixed_new_spline_coordinates_135471207108928, (coordinate_15 as f32));
            spline_values_17 = [0.1_f32, advanced_hermite(minecraft_new_combination_unfixed_new_spline_coordinates_135471207108928, spline_values_14, minecraft_new_combination_unfixed_new_spline_derivatives_135471207108872, (coordinate_15 as f32), (spline_index_16 as i32))];
            coordinate_18 = (input_135471207106448[index(p) as usize] as f32);
            spline_index_19 = binary_search(minecraft_new_combination_unfixed_new_spline_coordinates_135471207107632, (coordinate_18 as f32));
            spline_values_20 = [0_f32];
            coordinate_21 = (input_135471207105152[index(p) as usize] as f32);
            spline_index_22 = binary_search(minecraft_new_combination_unfixed_new_spline_coordinates_135471207106336, (coordinate_21 as f32));
            spline_values_23 = [0.1_f32, advanced_hermite(minecraft_new_combination_unfixed_new_spline_coordinates_135471207106336, spline_values_20, minecraft_new_combination_unfixed_new_spline_derivatives_135471207106280, (coordinate_21 as f32), (spline_index_22 as i32))];
            coordinate_24 = (input_135471207106448[index(p) as usize] as f32);
            spline_index_25 = binary_search(minecraft_new_combination_unfixed_new_spline_coordinates_135471207105040, (coordinate_24 as f32));
            spline_values_26 = [0_f32];
            coordinate_27 = (input_135471207103576[index(p) as usize] as f32);
            spline_index_28 = binary_search(minecraft_new_combination_unfixed_new_spline_coordinates_135471207104760, (coordinate_27 as f32));
            spline_values_29 = [0.1_f32, advanced_hermite(minecraft_new_combination_unfixed_new_spline_coordinates_135471207104760, spline_values_26, minecraft_new_combination_unfixed_new_spline_derivatives_135471207104704, (coordinate_27 as f32), (spline_index_28 as i32))];
            coordinate_30 = (input_135471207106448[index(p) as usize] as f32);
            spline_index_31 = binary_search(minecraft_new_combination_unfixed_new_spline_coordinates_135471207103464, (coordinate_30 as f32));
            spline_values_32 = [0_f32];
            coordinate_33 = (input_135471207144688[index(p) as usize] as f32);
            spline_index_34 = binary_search(minecraft_new_combination_unfixed_new_spline_coordinates_135471207103184, (coordinate_33 as f32));
            spline_values_35 = [advanced_hermite(minecraft_new_combination_unfixed_new_spline_coordinates_135471207107632, spline_values_17, minecraft_new_combination_unfixed_new_spline_derivatives_135471207107576, (coordinate_18 as f32), (spline_index_19 as i32)), advanced_hermite(minecraft_new_combination_unfixed_new_spline_coordinates_135471207105040, spline_values_23, minecraft_new_combination_unfixed_new_spline_derivatives_135471207104984, (coordinate_24 as f32), (spline_index_25 as i32)), advanced_hermite(minecraft_new_combination_unfixed_new_spline_coordinates_135471207103464, spline_values_29, minecraft_new_combination_unfixed_new_spline_derivatives_135471207103408, (coordinate_30 as f32), (spline_index_31 as i32)), advanced_hermite(minecraft_new_combination_unfixed_new_spline_coordinates_135471207103184, spline_values_32, minecraft_new_combination_unfixed_new_spline_derivatives_135471207103128, (coordinate_33 as f32), (spline_index_34 as i32))];
            coordinate_36 = (input_135471207102032[index(p) as usize] as f32);
            spline_index_37 = binary_search(minecraft_new_combination_unfixed_new_spline_coordinates_135471207102848, (coordinate_36 as f32));
            spline_values_38 = [0_f32];
            coordinate_39 = (input_135471207098792[index(p) as usize] as f32);
            spline_index_40 = binary_search(minecraft_new_combination_unfixed_new_spline_coordinates_135471207101920, (coordinate_39 as f32));
            spline_values_41 = [0.1_f32, advanced_hermite(minecraft_new_combination_unfixed_new_spline_coordinates_135471207101920, spline_values_38, minecraft_new_combination_unfixed_new_spline_derivatives_135471207101864, (coordinate_39 as f32), (spline_index_40 as i32))];
            coordinate_42 = (input_135471207106448[index(p) as usize] as f32);
            spline_index_43 = binary_search(minecraft_new_combination_unfixed_new_spline_coordinates_135471207098680, (coordinate_42 as f32));
            spline_values_44 = [0_f32];
            coordinate_45 = (input_135471207096712[index(p) as usize] as f32);
            spline_index_46 = binary_search(minecraft_new_combination_unfixed_new_spline_coordinates_135471207098400, (coordinate_45 as f32));
            spline_values_47 = [0.1_f32, advanced_hermite(minecraft_new_combination_unfixed_new_spline_coordinates_135471207098400, spline_values_44, minecraft_new_combination_unfixed_new_spline_derivatives_135471207098344, (coordinate_45 as f32), (spline_index_46 as i32))];
            coordinate_48 = (input_135471207106448[index(p) as usize] as f32);
            spline_index_49 = binary_search(minecraft_new_combination_unfixed_new_spline_coordinates_135471207096600, (coordinate_48 as f32));
            spline_values_50 = [0_f32];
            coordinate_51 = (input_135471207094632[index(p) as usize] as f32);
            spline_index_52 = binary_search(minecraft_new_combination_unfixed_new_spline_coordinates_135471207096320, (coordinate_51 as f32));
            spline_values_53 = [0.1_f32, advanced_hermite(minecraft_new_combination_unfixed_new_spline_coordinates_135471207096320, spline_values_50, minecraft_new_combination_unfixed_new_spline_derivatives_135471207096264, (coordinate_51 as f32), (spline_index_52 as i32))];
            coordinate_54 = (input_135471207106448[index(p) as usize] as f32);
            spline_index_55 = binary_search(minecraft_new_combination_unfixed_new_spline_coordinates_135471207094520, (coordinate_54 as f32));
            spline_values_56 = [advanced_hermite(minecraft_new_combination_unfixed_new_spline_coordinates_135471207098680, spline_values_41, minecraft_new_combination_unfixed_new_spline_derivatives_135471207098624, (coordinate_42 as f32), (spline_index_43 as i32)), advanced_hermite(minecraft_new_combination_unfixed_new_spline_coordinates_135471207096600, spline_values_47, minecraft_new_combination_unfixed_new_spline_derivatives_135471207096544, (coordinate_48 as f32), (spline_index_49 as i32)), advanced_hermite(minecraft_new_combination_unfixed_new_spline_coordinates_135471207094520, spline_values_53, minecraft_new_combination_unfixed_new_spline_derivatives_135471207094464, (coordinate_54 as f32), (spline_index_55 as i32)), advanced_hermite(minecraft_new_combination_unfixed_new_spline_coordinates_135471207103184, spline_values_32, minecraft_new_combination_unfixed_new_spline_derivatives_135471207103128, (coordinate_33 as f32), (spline_index_34 as i32))];
            coordinate_57 = (input_135471207102032[index(p) as usize] as f32);
            spline_index_58 = binary_search(minecraft_new_combination_unfixed_new_spline_coordinates_135471207094240, (coordinate_57 as f32));
            spline_values_59 = [advanced_hermite(minecraft_new_combination_unfixed_new_spline_coordinates_135471207102848, spline_values_35, minecraft_new_combination_unfixed_new_spline_derivatives_135471207102792, (coordinate_36 as f32), (spline_index_37 as i32)), advanced_hermite(minecraft_new_combination_unfixed_new_spline_coordinates_135471207094240, spline_values_56, minecraft_new_combination_unfixed_new_spline_derivatives_135471207094184, (coordinate_57 as f32), (spline_index_58 as i32))];
            coordinate_60 = (input_135471207092776[index(p) as usize] as f32);
            spline_index_61 = binary_search(minecraft_new_combination_unfixed_new_spline_coordinates_135471207093960, (coordinate_60 as f32));
            spline_values_62 = [advanced_hermite(minecraft_new_combination_unfixed_new_spline_coordinates_135471207109208, spline_values_11, minecraft_new_combination_unfixed_new_spline_derivatives_135471207109152, (coordinate_12 as f32), (spline_index_13 as i32)), advanced_hermite(minecraft_new_combination_unfixed_new_spline_coordinates_135471207093960, spline_values_59, minecraft_new_combination_unfixed_new_spline_derivatives_135471207093904, (coordinate_60 as f32), (spline_index_61 as i32))];
            coordinate_63 = (input_135471207145616[index(p) as usize] as f32);
            spline_index_64 = binary_search(minecraft_new_combination_unfixed_new_spline_coordinates_135471207092664, (coordinate_63 as f32));
            spline_result_f32_65 = advanced_hermite(minecraft_new_combination_unfixed_new_spline_coordinates_135471207092664, spline_values_62, minecraft_new_combination_unfixed_new_spline_derivatives_135471207092608, (coordinate_63 as f32), (spline_index_64 as i32));
            spline_result_f64_66 = (spline_result_f32_65 as f64);
            out[index(p)] = min(spline_result_f64_10, spline_result_f64_66);
        }
    }
    
    pub fn minecraft_jagged_37_d5x1x5(out: &mut [f64; 25], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, perm_table_minecraft_jagged_0_octave__16: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__16: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__15: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__15: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__14: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__14: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__13: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__13: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__12: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__12: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__11: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__11: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__10: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__10: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__9: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__9: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__8: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__8: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__7: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__7: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__6: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__6: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__5: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__5: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__4: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__4: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__3: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__3: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__2: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__2: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__1: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__1: &PerlinNoiseSampler, host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut n: f64;
        for p in iter_3d(5_i32, 1_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * Vec3::new(600_f64, 0_f64, 600_f64)) + (pos3 * Vec3::new(2400_f64, 0_f64, 2400_f64)));
            n = 0_f64;
            n = (n + (perlin((rpos3 * 0.0000152587890625_f64), perm_table_minecraft_jagged_0_octave__16) * 0.5000076295109483_f64));
            n = (n + (perlin((rpos3 * 0.000015535383426170696_f64), perm_table_minecraft_jagged_1_octave__16) * 0.5000076295109483_f64));
            n = (n + (perlin((rpos3 * 0.000030517578125_f64), perm_table_minecraft_jagged_0_octave__15) * 0.2500038147554742_f64));
            n = (n + (perlin((rpos3 * 0.00003107076685234139_f64), perm_table_minecraft_jagged_1_octave__15) * 0.2500038147554742_f64));
            n = (n + (perlin((rpos3 * 0.00006103515625_f64), perm_table_minecraft_jagged_0_octave__14) * 0.1250019073777371_f64));
            n = (n + (perlin((rpos3 * 0.00006214153370468278_f64), perm_table_minecraft_jagged_1_octave__14) * 0.1250019073777371_f64));
            n = (n + (perlin((rpos3 * 0.0001220703125_f64), perm_table_minecraft_jagged_0_octave__13) * 0.06250095368886854_f64));
            n = (n + (perlin((rpos3 * 0.00012428306740936557_f64), perm_table_minecraft_jagged_1_octave__13) * 0.06250095368886854_f64));
            n = (n + (perlin((rpos3 * 0.000244140625_f64), perm_table_minecraft_jagged_0_octave__12) * 0.03125047684443427_f64));
            n = (n + (perlin((rpos3 * 0.00024856613481873114_f64), perm_table_minecraft_jagged_1_octave__12) * 0.03125047684443427_f64));
            n = (n + (perlin((rpos3 * 0.00048828125_f64), perm_table_minecraft_jagged_0_octave__11) * 0.015625238422217136_f64));
            n = (n + (perlin((rpos3 * 0.0004971322696374623_f64), perm_table_minecraft_jagged_1_octave__11) * 0.015625238422217136_f64));
            n = (n + (perlin((rpos3 * 0.0009765625_f64), perm_table_minecraft_jagged_0_octave__10) * 0.007812619211108568_f64));
            n = (n + (perlin((rpos3 * 0.0009942645392749245_f64), perm_table_minecraft_jagged_1_octave__10) * 0.007812619211108568_f64));
            n = (n + (perlin((rpos3 * 0.001953125_f64), perm_table_minecraft_jagged_0_octave__9) * 0.003906309605554284_f64));
            n = (n + (perlin((rpos3 * 0.001988529078549849_f64), perm_table_minecraft_jagged_1_octave__9) * 0.003906309605554284_f64));
            n = (n + (perlin((rpos3 * 0.00390625_f64), perm_table_minecraft_jagged_0_octave__8) * 0.001953154802777142_f64));
            n = (n + (perlin((rpos3 * 0.003977058157099698_f64), perm_table_minecraft_jagged_1_octave__8) * 0.001953154802777142_f64));
            n = (n + (perlin((rpos3 * 0.0078125_f64), perm_table_minecraft_jagged_0_octave__7) * 0.000976577401388571_f64));
            n = (n + (perlin((rpos3 * 0.007954116314199396_f64), perm_table_minecraft_jagged_1_octave__7) * 0.000976577401388571_f64));
            n = (n + (perlin((rpos3 * 0.015625_f64), perm_table_minecraft_jagged_0_octave__6) * 0.0004882887006942855_f64));
            n = (n + (perlin((rpos3 * 0.015908232628398793_f64), perm_table_minecraft_jagged_1_octave__6) * 0.0004882887006942855_f64));
            n = (n + (perlin((rpos3 * 0.03125_f64), perm_table_minecraft_jagged_0_octave__5) * 0.00024414435034714275_f64));
            n = (n + (perlin((rpos3 * 0.031816465256797585_f64), perm_table_minecraft_jagged_1_octave__5) * 0.00024414435034714275_f64));
            n = (n + (perlin((rpos3 * 0.0625_f64), perm_table_minecraft_jagged_0_octave__4) * 0.00012207217517357137_f64));
            n = (n + (perlin((rpos3 * 0.06363293051359517_f64), perm_table_minecraft_jagged_1_octave__4) * 0.00012207217517357137_f64));
            n = (n + (perlin((rpos3 * 0.125_f64), perm_table_minecraft_jagged_0_octave__3) * 0.00006103608758678569_f64));
            n = (n + (perlin((rpos3 * 0.12726586102719034_f64), perm_table_minecraft_jagged_1_octave__3) * 0.00006103608758678569_f64));
            n = (n + (perlin((rpos3 * 0.25_f64), perm_table_minecraft_jagged_0_octave__2) * 0.000030518043793392844_f64));
            n = (n + (perlin((rpos3 * 0.2545317220543807_f64), perm_table_minecraft_jagged_1_octave__2) * 0.000030518043793392844_f64));
            n = (n + (perlin((rpos3 * 0.5_f64), perm_table_minecraft_jagged_0_octave__1) * 0.000015259021896696422_f64));
            n = (n + (perlin((rpos3 * 0.5090634441087614_f64), perm_table_minecraft_jagged_1_octave__1) * 0.000015259021896696422_f64));
            out[index(p)] = (n * 1.568627450980392_f64);
        }
    }
    
    pub fn density_function_Abs_47_d5x263x5(out: &mut [f64; 6575], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207192184: &[f64; 25], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        for p in iter_3d(5_i32, 263_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            out[index(p)] = abs((input_135471207192184[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] + 0_f64));
        }
    }
    
    pub fn minecraft_realism_extreme_mountains_slope_height_d5x263x5(out: &mut [f64; 6575], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207164448: &[f64; 25], input_135471207163968: &[f64; 25], input_135471207116632: &[f64; 6575], input_135471207115648: &[f64; 6575], input_135471207114496: &[f64; 6575], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut spline_values_0: [f32; 17];
        let mut coordinate_1: f32;
        let mut spline_index_2: i32;
        let mut spline_result_f32_3: f32;
        let mut spline_result_f64_4: f64;
        let mut spline_values_5: [f32; 1];
        let mut coordinate_6: f32;
        let mut spline_index_7: i32;
        let mut spline_values_8: [f32; 1];
        let mut coordinate_9: f32;
        let mut spline_index_10: i32;
        let mut spline_values_11: [f32; 2];
        let mut coordinate_12: f32;
        let mut spline_index_13: i32;
        let mut spline_result_f32_14: f32;
        let mut spline_result_f64_15: f64;
        for p in iter_3d(5_i32, 263_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            spline_values_0 = [1_f32, 0.95238096_f32, 0.90909094_f32, 0.8333333_f32, 0.7692308_f32, 0.71428573_f32, 0.6666667_f32, 0.625_f32, 0.5882353_f32, 0.5555556_f32, 0.5263158_f32, 0.5_f32, 0.47619048_f32, 0.45454547_f32, 0.4347826_f32, 0.41666666_f32, 0.4_f32];
            coordinate_1 = (input_135471207116632[index(p) as usize] as f32);
            spline_index_2 = binary_search(minecraft_realism_extreme_mountains_slope_height_spline_coordinates_135471207119936, (coordinate_1 as f32));
            spline_result_f32_3 = advanced_hermite(minecraft_realism_extreme_mountains_slope_height_spline_coordinates_135471207119936, spline_values_0, minecraft_realism_extreme_mountains_slope_height_spline_derivatives_135471207119880, (coordinate_1 as f32), (spline_index_2 as i32));
            spline_result_f64_4 = (spline_result_f32_3 as f64);
            spline_values_5 = [0_f32];
            coordinate_6 = (input_135471207115648[index(p) as usize] as f32);
            spline_index_7 = binary_search(minecraft_realism_extreme_mountains_slope_height_spline_coordinates_135471207116408, (coordinate_6 as f32));
            spline_values_8 = [0_f32];
            coordinate_9 = (input_135471207115648[index(p) as usize] as f32);
            spline_index_10 = binary_search(minecraft_realism_extreme_mountains_slope_height_spline_coordinates_135471207115536, (coordinate_9 as f32));
            spline_values_11 = [advanced_hermite(minecraft_realism_extreme_mountains_slope_height_spline_coordinates_135471207116408, spline_values_5, minecraft_realism_extreme_mountains_slope_height_spline_derivatives_135471207116352, (coordinate_6 as f32), (spline_index_7 as i32)), advanced_hermite(minecraft_realism_extreme_mountains_slope_height_spline_coordinates_135471207115536, spline_values_8, minecraft_realism_extreme_mountains_slope_height_spline_derivatives_135471207115480, (coordinate_9 as f32), (spline_index_10 as i32))];
            coordinate_12 = (input_135471207114496[index(p) as usize] as f32);
            spline_index_13 = binary_search(minecraft_realism_extreme_mountains_slope_height_spline_coordinates_135471207115256, (coordinate_12 as f32));
            spline_result_f32_14 = advanced_hermite(minecraft_realism_extreme_mountains_slope_height_spline_coordinates_135471207115256, spline_values_11, minecraft_realism_extreme_mountains_slope_height_spline_derivatives_135471207115200, (coordinate_12 as f32), (spline_index_13 as i32));
            spline_result_f64_15 = (spline_result_f32_14 as f64);
            out[index(p)] = ((0_f64 + ((input_135471207164448[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * 0.2_f64) + 0.4_f64)) + (((input_135471207163968[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * spline_result_f64_4) * 0.9_f64) + spline_result_f64_15));
        }
    }
    
    pub fn minecraft_realism_hills_factor_original_d5x263x5(out: &mut [f64; 6575], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207178720: &[f64; 25], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        for p in iter_3d(5_i32, 263_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            out[index(p)] = input_135471207178720[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize];
        }
    }
    
    pub fn density_function_Add_40_d5x1x5(out: &mut [f64; 25], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207193224: &[f64; 25], input_135471207191928: &[f64; 25], input_135471207189672: &[f64; 25], input_135471207178096: &[f64; 25], input_135471207173912: &[f64; 25], input_135471207165768: &[f64; 25], input_135471207157624: &[f64; 25], input_135471207156048: &[f64; 25], input_135471207155680: &[f64; 25], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut spline_values_0: [f32; 1];
        let mut coordinate_1: f32;
        let mut spline_index_2: i32;
        let mut spline_values_3: [f32; 2];
        let mut coordinate_4: f32;
        let mut spline_index_5: i32;
        let mut spline_values_6: [f32; 1];
        let mut coordinate_7: f32;
        let mut spline_index_8: i32;
        let mut spline_values_9: [f32; 2];
        let mut coordinate_10: f32;
        let mut spline_index_11: i32;
        let mut spline_values_12: [f32; 1];
        let mut coordinate_13: f32;
        let mut spline_index_14: i32;
        let mut spline_values_15: [f32; 3];
        let mut coordinate_16: f32;
        let mut spline_index_17: i32;
        let mut spline_values_18: [f32; 1];
        let mut coordinate_19: f32;
        let mut spline_index_20: i32;
        let mut spline_values_21: [f32; 4];
        let mut coordinate_22: f32;
        let mut spline_index_23: i32;
        let mut spline_values_24: [f32; 1];
        let mut coordinate_25: f32;
        let mut spline_index_26: i32;
        let mut spline_values_27: [f32; 5];
        let mut coordinate_28: f32;
        let mut spline_index_29: i32;
        let mut spline_values_30: [f32; 1];
        let mut coordinate_31: f32;
        let mut spline_index_32: i32;
        let mut spline_values_33: [f32; 6];
        let mut coordinate_34: f32;
        let mut spline_index_35: i32;
        let mut spline_values_36: [f32; 7];
        let mut coordinate_37: f32;
        let mut spline_index_38: i32;
        let mut spline_result_f32_39: f32;
        let mut spline_result_f64_40: f64;
        let mut spline_values_41: [f32; 1];
        let mut coordinate_42: f32;
        let mut spline_index_43: i32;
        let mut spline_values_44: [f32; 2];
        let mut coordinate_45: f32;
        let mut spline_index_46: i32;
        let mut spline_values_47: [f32; 1];
        let mut coordinate_48: f32;
        let mut spline_index_49: i32;
        let mut spline_values_50: [f32; 2];
        let mut coordinate_51: f32;
        let mut spline_index_52: i32;
        let mut spline_values_53: [f32; 1];
        let mut coordinate_54: f32;
        let mut spline_index_55: i32;
        let mut spline_values_56: [f32; 3];
        let mut coordinate_57: f32;
        let mut spline_index_58: i32;
        let mut spline_values_59: [f32; 1];
        let mut coordinate_60: f32;
        let mut spline_index_61: i32;
        let mut spline_values_62: [f32; 4];
        let mut coordinate_63: f32;
        let mut spline_index_64: i32;
        let mut spline_values_65: [f32; 1];
        let mut coordinate_66: f32;
        let mut spline_index_67: i32;
        let mut spline_values_68: [f32; 5];
        let mut coordinate_69: f32;
        let mut spline_index_70: i32;
        let mut spline_values_71: [f32; 1];
        let mut coordinate_72: f32;
        let mut spline_index_73: i32;
        let mut spline_values_74: [f32; 6];
        let mut coordinate_75: f32;
        let mut spline_index_76: i32;
        let mut spline_values_77: [f32; 7];
        let mut coordinate_78: f32;
        let mut spline_index_79: i32;
        let mut spline_result_f32_80: f32;
        let mut spline_result_f64_81: f64;
        let mut spline_values_82: [f32; 2];
        let mut coordinate_83: f32;
        let mut spline_index_84: i32;
        let mut spline_result_f32_85: f32;
        let mut spline_result_f64_86: f64;
        for p in iter_3d(5_i32, 1_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            spline_values_0 = [0_f32];
            coordinate_1 = (input_135471207193224[index(p) as usize] as f32);
            spline_index_2 = binary_search(density_function_Add_40_spline_coordinates_135471207155008, (coordinate_1 as f32));
            spline_values_3 = [-0.9_f32, advanced_hermite(density_function_Add_40_spline_coordinates_135471207155008, spline_values_0, density_function_Add_40_spline_derivatives_135471207154952, (coordinate_1 as f32), (spline_index_2 as i32))];
            coordinate_4 = (input_135471207191928[index(p) as usize] as f32);
            spline_index_5 = binary_search(density_function_Add_40_spline_coordinates_135471207154672, (coordinate_4 as f32));
            spline_values_6 = [0_f32];
            coordinate_7 = (input_135471207189672[index(p) as usize] as f32);
            spline_index_8 = binary_search(density_function_Add_40_spline_coordinates_135471207154336, (coordinate_7 as f32));
            spline_values_9 = [advanced_hermite(density_function_Add_40_spline_coordinates_135471207154336, spline_values_6, density_function_Add_40_spline_derivatives_135471207154280, (coordinate_7 as f32), (spline_index_8 as i32)), advanced_hermite(density_function_Add_40_spline_coordinates_135471207155008, spline_values_0, density_function_Add_40_spline_derivatives_135471207154952, (coordinate_1 as f32), (spline_index_2 as i32))];
            coordinate_10 = (input_135471207191928[index(p) as usize] as f32);
            spline_index_11 = binary_search(density_function_Add_40_spline_coordinates_135471207154000, (coordinate_10 as f32));
            spline_values_12 = [0_f32];
            coordinate_13 = (input_135471207178096[index(p) as usize] as f32);
            spline_index_14 = binary_search(density_function_Add_40_spline_coordinates_135471207153720, (coordinate_13 as f32));
            spline_values_15 = [advanced_hermite(density_function_Add_40_spline_coordinates_135471207153720, spline_values_12, density_function_Add_40_spline_derivatives_135471207153664, (coordinate_13 as f32), (spline_index_14 as i32)), advanced_hermite(density_function_Add_40_spline_coordinates_135471207154336, spline_values_6, density_function_Add_40_spline_derivatives_135471207154280, (coordinate_7 as f32), (spline_index_8 as i32)), advanced_hermite(density_function_Add_40_spline_coordinates_135471207155008, spline_values_0, density_function_Add_40_spline_derivatives_135471207154952, (coordinate_1 as f32), (spline_index_2 as i32))];
            coordinate_16 = (input_135471207191928[index(p) as usize] as f32);
            spline_index_17 = binary_search(density_function_Add_40_spline_coordinates_135471207153384, (coordinate_16 as f32));
            spline_values_18 = [0_f32];
            coordinate_19 = (input_135471207173912[index(p) as usize] as f32);
            spline_index_20 = binary_search(density_function_Add_40_spline_coordinates_135471207153104, (coordinate_19 as f32));
            spline_values_21 = [advanced_hermite(density_function_Add_40_spline_coordinates_135471207153104, spline_values_18, density_function_Add_40_spline_derivatives_135471207153048, (coordinate_19 as f32), (spline_index_20 as i32)), advanced_hermite(density_function_Add_40_spline_coordinates_135471207153720, spline_values_12, density_function_Add_40_spline_derivatives_135471207153664, (coordinate_13 as f32), (spline_index_14 as i32)), advanced_hermite(density_function_Add_40_spline_coordinates_135471207154336, spline_values_6, density_function_Add_40_spline_derivatives_135471207154280, (coordinate_7 as f32), (spline_index_8 as i32)), advanced_hermite(density_function_Add_40_spline_coordinates_135471207155008, spline_values_0, density_function_Add_40_spline_derivatives_135471207154952, (coordinate_1 as f32), (spline_index_2 as i32))];
            coordinate_22 = (input_135471207191928[index(p) as usize] as f32);
            spline_index_23 = binary_search(density_function_Add_40_spline_coordinates_135471207152768, (coordinate_22 as f32));
            spline_values_24 = [0_f32];
            coordinate_25 = (input_135471207165768[index(p) as usize] as f32);
            spline_index_26 = binary_search(density_function_Add_40_spline_coordinates_135471207152488, (coordinate_25 as f32));
            spline_values_27 = [advanced_hermite(density_function_Add_40_spline_coordinates_135471207152488, spline_values_24, density_function_Add_40_spline_derivatives_135471207152432, (coordinate_25 as f32), (spline_index_26 as i32)), advanced_hermite(density_function_Add_40_spline_coordinates_135471207153104, spline_values_18, density_function_Add_40_spline_derivatives_135471207153048, (coordinate_19 as f32), (spline_index_20 as i32)), advanced_hermite(density_function_Add_40_spline_coordinates_135471207153720, spline_values_12, density_function_Add_40_spline_derivatives_135471207153664, (coordinate_13 as f32), (spline_index_14 as i32)), advanced_hermite(density_function_Add_40_spline_coordinates_135471207154336, spline_values_6, density_function_Add_40_spline_derivatives_135471207154280, (coordinate_7 as f32), (spline_index_8 as i32)), advanced_hermite(density_function_Add_40_spline_coordinates_135471207155008, spline_values_0, density_function_Add_40_spline_derivatives_135471207154952, (coordinate_1 as f32), (spline_index_2 as i32))];
            coordinate_28 = (input_135471207191928[index(p) as usize] as f32);
            spline_index_29 = binary_search(density_function_Add_40_spline_coordinates_135471207152152, (coordinate_28 as f32));
            spline_values_30 = [0_f32];
            coordinate_31 = (input_135471207157624[index(p) as usize] as f32);
            spline_index_32 = binary_search(density_function_Add_40_spline_coordinates_135471207151872, (coordinate_31 as f32));
            spline_values_33 = [advanced_hermite(density_function_Add_40_spline_coordinates_135471207151872, spline_values_30, density_function_Add_40_spline_derivatives_135471207151816, (coordinate_31 as f32), (spline_index_32 as i32)), advanced_hermite(density_function_Add_40_spline_coordinates_135471207152488, spline_values_24, density_function_Add_40_spline_derivatives_135471207152432, (coordinate_25 as f32), (spline_index_26 as i32)), advanced_hermite(density_function_Add_40_spline_coordinates_135471207153104, spline_values_18, density_function_Add_40_spline_derivatives_135471207153048, (coordinate_19 as f32), (spline_index_20 as i32)), advanced_hermite(density_function_Add_40_spline_coordinates_135471207153720, spline_values_12, density_function_Add_40_spline_derivatives_135471207153664, (coordinate_13 as f32), (spline_index_14 as i32)), advanced_hermite(density_function_Add_40_spline_coordinates_135471207154336, spline_values_6, density_function_Add_40_spline_derivatives_135471207154280, (coordinate_7 as f32), (spline_index_8 as i32)), advanced_hermite(density_function_Add_40_spline_coordinates_135471207155008, spline_values_0, density_function_Add_40_spline_derivatives_135471207154952, (coordinate_1 as f32), (spline_index_2 as i32))];
            coordinate_34 = (input_135471207191928[index(p) as usize] as f32);
            spline_index_35 = binary_search(density_function_Add_40_spline_coordinates_135471207151536, (coordinate_34 as f32));
            spline_values_36 = [advanced_hermite(density_function_Add_40_spline_coordinates_135471207154672, spline_values_3, density_function_Add_40_spline_derivatives_135471207154616, (coordinate_4 as f32), (spline_index_5 as i32)), advanced_hermite(density_function_Add_40_spline_coordinates_135471207154000, spline_values_9, density_function_Add_40_spline_derivatives_135471207153944, (coordinate_10 as f32), (spline_index_11 as i32)), advanced_hermite(density_function_Add_40_spline_coordinates_135471207153384, spline_values_15, density_function_Add_40_spline_derivatives_135471207153328, (coordinate_16 as f32), (spline_index_17 as i32)), advanced_hermite(density_function_Add_40_spline_coordinates_135471207153384, spline_values_15, density_function_Add_40_spline_derivatives_135471207153328, (coordinate_16 as f32), (spline_index_17 as i32)), advanced_hermite(density_function_Add_40_spline_coordinates_135471207152768, spline_values_21, density_function_Add_40_spline_derivatives_135471207152712, (coordinate_22 as f32), (spline_index_23 as i32)), advanced_hermite(density_function_Add_40_spline_coordinates_135471207152152, spline_values_27, density_function_Add_40_spline_derivatives_135471207152096, (coordinate_28 as f32), (spline_index_29 as i32)), advanced_hermite(density_function_Add_40_spline_coordinates_135471207151536, spline_values_33, density_function_Add_40_spline_derivatives_135471207151480, (coordinate_34 as f32), (spline_index_35 as i32))];
            coordinate_37 = (input_135471207156048[index(p) as usize] as f32);
            spline_index_38 = binary_search(density_function_Add_40_spline_coordinates_135471207151256, (coordinate_37 as f32));
            spline_result_f32_39 = advanced_hermite(density_function_Add_40_spline_coordinates_135471207151256, spline_values_36, density_function_Add_40_spline_derivatives_135471207151200, (coordinate_37 as f32), (spline_index_38 as i32));
            spline_result_f64_40 = (spline_result_f32_39 as f64);
            spline_values_41 = [0_f32];
            coordinate_42 = (input_135471207193224[index(p) as usize] as f32);
            spline_index_43 = binary_search(density_function_Add_40_spline_coordinates_135471207150808, (coordinate_42 as f32));
            spline_values_44 = [-0.9_f32, advanced_hermite(density_function_Add_40_spline_coordinates_135471207150808, spline_values_41, density_function_Add_40_spline_derivatives_135471207150752, (coordinate_42 as f32), (spline_index_43 as i32))];
            coordinate_45 = (input_135471207191928[index(p) as usize] as f32);
            spline_index_46 = binary_search(density_function_Add_40_spline_coordinates_135471207150528, (coordinate_45 as f32));
            spline_values_47 = [0_f32];
            coordinate_48 = (input_135471207189672[index(p) as usize] as f32);
            spline_index_49 = binary_search(density_function_Add_40_spline_coordinates_135471207150248, (coordinate_48 as f32));
            spline_values_50 = [advanced_hermite(density_function_Add_40_spline_coordinates_135471207150248, spline_values_47, density_function_Add_40_spline_derivatives_135471207150192, (coordinate_48 as f32), (spline_index_49 as i32)), advanced_hermite(density_function_Add_40_spline_coordinates_135471207150808, spline_values_41, density_function_Add_40_spline_derivatives_135471207150752, (coordinate_42 as f32), (spline_index_43 as i32))];
            coordinate_51 = (input_135471207191928[index(p) as usize] as f32);
            spline_index_52 = binary_search(density_function_Add_40_spline_coordinates_135471207149968, (coordinate_51 as f32));
            spline_values_53 = [0_f32];
            coordinate_54 = (input_135471207178096[index(p) as usize] as f32);
            spline_index_55 = binary_search(density_function_Add_40_spline_coordinates_135471207149688, (coordinate_54 as f32));
            spline_values_56 = [advanced_hermite(density_function_Add_40_spline_coordinates_135471207149688, spline_values_53, density_function_Add_40_spline_derivatives_135471207149632, (coordinate_54 as f32), (spline_index_55 as i32)), advanced_hermite(density_function_Add_40_spline_coordinates_135471207150248, spline_values_47, density_function_Add_40_spline_derivatives_135471207150192, (coordinate_48 as f32), (spline_index_49 as i32)), advanced_hermite(density_function_Add_40_spline_coordinates_135471207150808, spline_values_41, density_function_Add_40_spline_derivatives_135471207150752, (coordinate_42 as f32), (spline_index_43 as i32))];
            coordinate_57 = (input_135471207191928[index(p) as usize] as f32);
            spline_index_58 = binary_search(density_function_Add_40_spline_coordinates_135471207149408, (coordinate_57 as f32));
            spline_values_59 = [0_f32];
            coordinate_60 = (input_135471207173912[index(p) as usize] as f32);
            spline_index_61 = binary_search(density_function_Add_40_spline_coordinates_135471207149128, (coordinate_60 as f32));
            spline_values_62 = [advanced_hermite(density_function_Add_40_spline_coordinates_135471207149128, spline_values_59, density_function_Add_40_spline_derivatives_135471207149072, (coordinate_60 as f32), (spline_index_61 as i32)), advanced_hermite(density_function_Add_40_spline_coordinates_135471207149688, spline_values_53, density_function_Add_40_spline_derivatives_135471207149632, (coordinate_54 as f32), (spline_index_55 as i32)), advanced_hermite(density_function_Add_40_spline_coordinates_135471207150248, spline_values_47, density_function_Add_40_spline_derivatives_135471207150192, (coordinate_48 as f32), (spline_index_49 as i32)), advanced_hermite(density_function_Add_40_spline_coordinates_135471207150808, spline_values_41, density_function_Add_40_spline_derivatives_135471207150752, (coordinate_42 as f32), (spline_index_43 as i32))];
            coordinate_63 = (input_135471207191928[index(p) as usize] as f32);
            spline_index_64 = binary_search(density_function_Add_40_spline_coordinates_135471207148848, (coordinate_63 as f32));
            spline_values_65 = [0_f32];
            coordinate_66 = (input_135471207165768[index(p) as usize] as f32);
            spline_index_67 = binary_search(density_function_Add_40_spline_coordinates_135471207148568, (coordinate_66 as f32));
            spline_values_68 = [advanced_hermite(density_function_Add_40_spline_coordinates_135471207148568, spline_values_65, density_function_Add_40_spline_derivatives_135471207148512, (coordinate_66 as f32), (spline_index_67 as i32)), advanced_hermite(density_function_Add_40_spline_coordinates_135471207149128, spline_values_59, density_function_Add_40_spline_derivatives_135471207149072, (coordinate_60 as f32), (spline_index_61 as i32)), advanced_hermite(density_function_Add_40_spline_coordinates_135471207149688, spline_values_53, density_function_Add_40_spline_derivatives_135471207149632, (coordinate_54 as f32), (spline_index_55 as i32)), advanced_hermite(density_function_Add_40_spline_coordinates_135471207150248, spline_values_47, density_function_Add_40_spline_derivatives_135471207150192, (coordinate_48 as f32), (spline_index_49 as i32)), advanced_hermite(density_function_Add_40_spline_coordinates_135471207150808, spline_values_41, density_function_Add_40_spline_derivatives_135471207150752, (coordinate_42 as f32), (spline_index_43 as i32))];
            coordinate_69 = (input_135471207191928[index(p) as usize] as f32);
            spline_index_70 = binary_search(density_function_Add_40_spline_coordinates_135471207148288, (coordinate_69 as f32));
            spline_values_71 = [0_f32];
            coordinate_72 = (input_135471207157624[index(p) as usize] as f32);
            spline_index_73 = binary_search(density_function_Add_40_spline_coordinates_135471207148008, (coordinate_72 as f32));
            spline_values_74 = [advanced_hermite(density_function_Add_40_spline_coordinates_135471207148008, spline_values_71, density_function_Add_40_spline_derivatives_135471207147952, (coordinate_72 as f32), (spline_index_73 as i32)), advanced_hermite(density_function_Add_40_spline_coordinates_135471207148568, spline_values_65, density_function_Add_40_spline_derivatives_135471207148512, (coordinate_66 as f32), (spline_index_67 as i32)), advanced_hermite(density_function_Add_40_spline_coordinates_135471207149128, spline_values_59, density_function_Add_40_spline_derivatives_135471207149072, (coordinate_60 as f32), (spline_index_61 as i32)), advanced_hermite(density_function_Add_40_spline_coordinates_135471207149688, spline_values_53, density_function_Add_40_spline_derivatives_135471207149632, (coordinate_54 as f32), (spline_index_55 as i32)), advanced_hermite(density_function_Add_40_spline_coordinates_135471207150248, spline_values_47, density_function_Add_40_spline_derivatives_135471207150192, (coordinate_48 as f32), (spline_index_49 as i32)), advanced_hermite(density_function_Add_40_spline_coordinates_135471207150808, spline_values_41, density_function_Add_40_spline_derivatives_135471207150752, (coordinate_42 as f32), (spline_index_43 as i32))];
            coordinate_75 = (input_135471207191928[index(p) as usize] as f32);
            spline_index_76 = binary_search(density_function_Add_40_spline_coordinates_135471207147728, (coordinate_75 as f32));
            spline_values_77 = [advanced_hermite(density_function_Add_40_spline_coordinates_135471207150528, spline_values_44, density_function_Add_40_spline_derivatives_135471207150472, (coordinate_45 as f32), (spline_index_46 as i32)), advanced_hermite(density_function_Add_40_spline_coordinates_135471207149968, spline_values_50, density_function_Add_40_spline_derivatives_135471207149912, (coordinate_51 as f32), (spline_index_52 as i32)), advanced_hermite(density_function_Add_40_spline_coordinates_135471207149408, spline_values_56, density_function_Add_40_spline_derivatives_135471207149352, (coordinate_57 as f32), (spline_index_58 as i32)), advanced_hermite(density_function_Add_40_spline_coordinates_135471207149408, spline_values_56, density_function_Add_40_spline_derivatives_135471207149352, (coordinate_57 as f32), (spline_index_58 as i32)), advanced_hermite(density_function_Add_40_spline_coordinates_135471207148848, spline_values_62, density_function_Add_40_spline_derivatives_135471207148792, (coordinate_63 as f32), (spline_index_64 as i32)), advanced_hermite(density_function_Add_40_spline_coordinates_135471207148288, spline_values_68, density_function_Add_40_spline_derivatives_135471207148232, (coordinate_69 as f32), (spline_index_70 as i32)), advanced_hermite(density_function_Add_40_spline_coordinates_135471207147728, spline_values_74, density_function_Add_40_spline_derivatives_135471207147672, (coordinate_75 as f32), (spline_index_76 as i32))];
            coordinate_78 = (input_135471207156048[index(p) as usize] as f32);
            spline_index_79 = binary_search(density_function_Add_40_spline_coordinates_135471207147448, (coordinate_78 as f32));
            spline_result_f32_80 = advanced_hermite(density_function_Add_40_spline_coordinates_135471207147448, spline_values_77, density_function_Add_40_spline_derivatives_135471207147392, (coordinate_78 as f32), (spline_index_79 as i32));
            spline_result_f64_81 = (spline_result_f32_80 as f64);
            spline_values_82 = [0_f32, -1.5_f32];
            coordinate_83 = (input_135471207155680[index(p) as usize] as f32);
            spline_index_84 = binary_search(density_function_Add_40_spline_coordinates_135471207147056, (coordinate_83 as f32));
            spline_result_f32_85 = advanced_hermite(density_function_Add_40_spline_coordinates_135471207147056, spline_values_82, density_function_Add_40_spline_derivatives_135471207147000, (coordinate_83 as f32), (spline_index_84 as i32));
            spline_result_f64_86 = (spline_result_f32_85 as f64);
            out[index(p)] = (spline_result_f64_40 + (spline_result_f64_81 * spline_result_f64_86));
        }
    }
    
    pub fn final_density_with_beardify_d16x2096x16(out: &mut [f64; 536576], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207090832: &[f64; 6575], input_135471207088808: &[f64; 536576], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        for p in iter_3d(16_i32, 2096_i32, 16_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            out[index(p)] = (interpolate(input_135471207090832[cornerx0y0z0_8(pos3, 5_i32, 263_i32, 5_i32) as usize], input_135471207090832[cornerx4y0z0_8(pos3, 5_i32, 263_i32, 5_i32) as usize], input_135471207090832[cornerx0y8z0_8(pos3, 5_i32, 263_i32, 5_i32) as usize], input_135471207090832[cornerx4y8z0_8(pos3, 5_i32, 263_i32, 5_i32) as usize], input_135471207090832[cornerx0y0z4_8(pos3, 5_i32, 263_i32, 5_i32) as usize], input_135471207090832[cornerx4y0z4_8(pos3, 5_i32, 263_i32, 5_i32) as usize], input_135471207090832[cornerx0y8z4_8(pos3, 5_i32, 263_i32, 5_i32) as usize], input_135471207090832[cornerx4y8z4_8(pos3, 5_i32, 263_i32, 5_i32) as usize], xfract4(pos3), yfract8(pos3), zfract4(pos3)) + input_135471207088808[index(p) as usize]);
        }
    }
    
    pub fn minecraft_new_combination_fixed_d5x263x5(out: &mut [f64; 6575], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207092072: &[f64; 6575], input_135471207091200: &[f64; 6575], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut spline_values_0: [f32; 1];
        let mut coordinate_1: f32;
        let mut spline_index_2: i32;
        let mut spline_values_3: [f32; 2];
        let mut coordinate_4: f32;
        let mut spline_index_5: i32;
        let mut spline_result_f32_6: f32;
        let mut spline_result_f64_7: f64;
        for p in iter_3d(5_i32, 263_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            spline_values_0 = [0_f32];
            coordinate_1 = (input_135471207092072[index(p) as usize] as f32);
            spline_index_2 = binary_search(minecraft_new_combination_fixed_spline_coordinates_135471207198000, (coordinate_1 as f32));
            spline_values_3 = [0.1_f32, advanced_hermite(minecraft_new_combination_fixed_spline_coordinates_135471207198000, spline_values_0, minecraft_new_combination_fixed_spline_derivatives_135471207197944, (coordinate_1 as f32), (spline_index_2 as i32))];
            coordinate_4 = (input_135471207091200[index(p) as usize] as f32);
            spline_index_5 = binary_search(minecraft_new_combination_fixed_spline_coordinates_135471207091960, (coordinate_4 as f32));
            spline_result_f32_6 = advanced_hermite(minecraft_new_combination_fixed_spline_coordinates_135471207091960, spline_values_3, minecraft_new_combination_fixed_spline_derivatives_135471207091904, (coordinate_4 as f32), (spline_index_5 as i32));
            spline_result_f64_7 = (spline_result_f32_6 as f64);
            out[index(p)] = spline_result_f64_7;
        }
    }
    
    pub fn density_function_Multiply_24_d5x1x5(out: &mut [f64; 25], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207170224: &[f64; 25], input_135471207172112: &[f64; 25], input_135471207168896: &[f64; 25], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut var_0: f64;
        let mut var_1: f64;
        for p in iter_3d(5_i32, 1_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            var_0 = (input_135471207170224[index(p) as usize] + (input_135471207172112[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * -1_f64));
            var_1 = (input_135471207168896[index(p) as usize] + (input_135471207172112[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * -1_f64));
            out[index(p)] = ((((var_0 * var_0) + (var_1 * var_1)) * 5000_f64) * 0.2_f64);
        }
    }
    
    pub fn density_function_ShiftedNoise_14_d5x1x5(out: &mut [f64; 25], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, perm_table_minecraft_gravel_0_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__5: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__5: &PerlinNoiseSampler, host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        for p in iter_3d(5_i32, 1_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            out[index(p)] = density_function_ShiftedNoise_14_d5x1x5_minecraft_gravel_shifted_15(((rpos3 * Vec3::new(0.25_f64, 0_f64, 0.25_f64)) + Vec3::new(1_f64, 0_f64, 0_f64)), perm_table_minecraft_gravel_0_octave__8, perm_table_minecraft_gravel_1_octave__8, perm_table_minecraft_gravel_0_octave__7, perm_table_minecraft_gravel_1_octave__7, perm_table_minecraft_gravel_0_octave__6, perm_table_minecraft_gravel_1_octave__6, perm_table_minecraft_gravel_0_octave__5, perm_table_minecraft_gravel_1_octave__5, host_consts);
        }
    }
    
    pub fn minecraft_cave_layer_71_d5x263x5(out: &mut [f64; 6575], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, perm_table_minecraft_cave_layer_0_octave__8: &PerlinNoiseSampler, perm_table_minecraft_cave_layer_1_octave__8: &PerlinNoiseSampler, host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut n: f64;
        for p in iter_3d(5_i32, 263_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * Vec3::new(10_f64, 4_f64, 10_f64)) + (pos3 * Vec3::new(40_f64, 32_f64, 40_f64)));
            n = 0_f64;
            n = (n + (perlin((rpos3 * 0.00390625_f64), perm_table_minecraft_cave_layer_0_octave__8) * 1_f64));
            n = (n + (perlin((rpos3 * 0.003977058157099698_f64), perm_table_minecraft_cave_layer_1_octave__8) * 1_f64));
            out[index(p)] = (n * 0.8333333333333333_f64);
        }
    }
    
    pub fn minecraft_jagged_72_d5x1x5(out: &mut [f64; 25], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, perm_table_minecraft_jagged_0_octave__16: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__16: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__15: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__15: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__14: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__14: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__13: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__13: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__12: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__12: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__11: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__11: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__10: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__10: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__9: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__9: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__8: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__8: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__7: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__7: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__6: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__6: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__5: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__5: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__4: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__4: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__3: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__3: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__2: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__2: &PerlinNoiseSampler, perm_table_minecraft_jagged_0_octave__1: &PerlinNoiseSampler, perm_table_minecraft_jagged_1_octave__1: &PerlinNoiseSampler, host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut n: f64;
        for p in iter_3d(5_i32, 1_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * Vec3::new(1000_f64, 0_f64, 1000_f64)) + (pos3 * Vec3::new(4000_f64, 0_f64, 4000_f64)));
            n = 0_f64;
            n = (n + (perlin((rpos3 * 0.0000152587890625_f64), perm_table_minecraft_jagged_0_octave__16) * 0.5000076295109483_f64));
            n = (n + (perlin((rpos3 * 0.000015535383426170696_f64), perm_table_minecraft_jagged_1_octave__16) * 0.5000076295109483_f64));
            n = (n + (perlin((rpos3 * 0.000030517578125_f64), perm_table_minecraft_jagged_0_octave__15) * 0.2500038147554742_f64));
            n = (n + (perlin((rpos3 * 0.00003107076685234139_f64), perm_table_minecraft_jagged_1_octave__15) * 0.2500038147554742_f64));
            n = (n + (perlin((rpos3 * 0.00006103515625_f64), perm_table_minecraft_jagged_0_octave__14) * 0.1250019073777371_f64));
            n = (n + (perlin((rpos3 * 0.00006214153370468278_f64), perm_table_minecraft_jagged_1_octave__14) * 0.1250019073777371_f64));
            n = (n + (perlin((rpos3 * 0.0001220703125_f64), perm_table_minecraft_jagged_0_octave__13) * 0.06250095368886854_f64));
            n = (n + (perlin((rpos3 * 0.00012428306740936557_f64), perm_table_minecraft_jagged_1_octave__13) * 0.06250095368886854_f64));
            n = (n + (perlin((rpos3 * 0.000244140625_f64), perm_table_minecraft_jagged_0_octave__12) * 0.03125047684443427_f64));
            n = (n + (perlin((rpos3 * 0.00024856613481873114_f64), perm_table_minecraft_jagged_1_octave__12) * 0.03125047684443427_f64));
            n = (n + (perlin((rpos3 * 0.00048828125_f64), perm_table_minecraft_jagged_0_octave__11) * 0.015625238422217136_f64));
            n = (n + (perlin((rpos3 * 0.0004971322696374623_f64), perm_table_minecraft_jagged_1_octave__11) * 0.015625238422217136_f64));
            n = (n + (perlin((rpos3 * 0.0009765625_f64), perm_table_minecraft_jagged_0_octave__10) * 0.007812619211108568_f64));
            n = (n + (perlin((rpos3 * 0.0009942645392749245_f64), perm_table_minecraft_jagged_1_octave__10) * 0.007812619211108568_f64));
            n = (n + (perlin((rpos3 * 0.001953125_f64), perm_table_minecraft_jagged_0_octave__9) * 0.003906309605554284_f64));
            n = (n + (perlin((rpos3 * 0.001988529078549849_f64), perm_table_minecraft_jagged_1_octave__9) * 0.003906309605554284_f64));
            n = (n + (perlin((rpos3 * 0.00390625_f64), perm_table_minecraft_jagged_0_octave__8) * 0.001953154802777142_f64));
            n = (n + (perlin((rpos3 * 0.003977058157099698_f64), perm_table_minecraft_jagged_1_octave__8) * 0.001953154802777142_f64));
            n = (n + (perlin((rpos3 * 0.0078125_f64), perm_table_minecraft_jagged_0_octave__7) * 0.000976577401388571_f64));
            n = (n + (perlin((rpos3 * 0.007954116314199396_f64), perm_table_minecraft_jagged_1_octave__7) * 0.000976577401388571_f64));
            n = (n + (perlin((rpos3 * 0.015625_f64), perm_table_minecraft_jagged_0_octave__6) * 0.0004882887006942855_f64));
            n = (n + (perlin((rpos3 * 0.015908232628398793_f64), perm_table_minecraft_jagged_1_octave__6) * 0.0004882887006942855_f64));
            n = (n + (perlin((rpos3 * 0.03125_f64), perm_table_minecraft_jagged_0_octave__5) * 0.00024414435034714275_f64));
            n = (n + (perlin((rpos3 * 0.031816465256797585_f64), perm_table_minecraft_jagged_1_octave__5) * 0.00024414435034714275_f64));
            n = (n + (perlin((rpos3 * 0.0625_f64), perm_table_minecraft_jagged_0_octave__4) * 0.00012207217517357137_f64));
            n = (n + (perlin((rpos3 * 0.06363293051359517_f64), perm_table_minecraft_jagged_1_octave__4) * 0.00012207217517357137_f64));
            n = (n + (perlin((rpos3 * 0.125_f64), perm_table_minecraft_jagged_0_octave__3) * 0.00006103608758678569_f64));
            n = (n + (perlin((rpos3 * 0.12726586102719034_f64), perm_table_minecraft_jagged_1_octave__3) * 0.00006103608758678569_f64));
            n = (n + (perlin((rpos3 * 0.25_f64), perm_table_minecraft_jagged_0_octave__2) * 0.000030518043793392844_f64));
            n = (n + (perlin((rpos3 * 0.2545317220543807_f64), perm_table_minecraft_jagged_1_octave__2) * 0.000030518043793392844_f64));
            n = (n + (perlin((rpos3 * 0.5_f64), perm_table_minecraft_jagged_0_octave__1) * 0.000015259021896696422_f64));
            n = (n + (perlin((rpos3 * 0.5090634441087614_f64), perm_table_minecraft_jagged_1_octave__1) * 0.000015259021896696422_f64));
            out[index(p)] = (n * 1.568627450980392_f64);
        }
    }
    
    pub fn minecraft_realism_extreme_mountains_factor_original_d5x1x5(out: &mut [f64; 25], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207163968: &[f64; 25], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        for p in iter_3d(5_i32, 1_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            out[index(p)] = input_135471207163968[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize];
        }
    }
    
    pub fn minecraft_caves_old_medium_d5x263x5(out: &mut [f64; 6575], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207105408: &[f64; 6575], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        for p in iter_3d(5_i32, 263_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            out[index(p)] = input_135471207105408[index(p) as usize];
        }
    }
    
    pub fn density_function_Abs_4_d5x1x5(out: &mut [f64; 25], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207192184: &[f64; 25], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        for p in iter_3d(5_i32, 1_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            out[index(p)] = abs((input_135471207192184[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] + 0_f64));
        }
    }
    
    pub fn minecraft_realism_mountains_factor_original_d5x1x5(out: &mut [f64; 25], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207172112: &[f64; 25], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        for p in iter_3d(5_i32, 1_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            out[index(p)] = input_135471207172112[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize];
        }
    }
    
    pub fn minecraft_realism_hills_factor_original_d5x1x5(out: &mut [f64; 25], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207178720: &[f64; 25], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        for p in iter_3d(5_i32, 1_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            out[index(p)] = input_135471207178720[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize];
        }
    }
    
    pub fn density_function_Multiply_13_d5x1x5(out: &mut [f64; 25], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207182464: &[f64; 25], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut spline_values_0: [f32; 20];
        let mut coordinate_1: f32;
        let mut spline_index_2: i32;
        let mut spline_result_f32_3: f32;
        let mut spline_result_f64_4: f64;
        for p in iter_3d(5_i32, 1_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            spline_values_0 = [0_f32, 0.31622776_f32, 0.4472136_f32, 0.5477226_f32, 0.6324555_f32, 0.70710677_f32, 0.7745967_f32, 0.83666_f32, 0.8944272_f32, 0.9486833_f32, 1_f32, 1.4142135_f32, 1.7320508_f32, 2_f32, 2.236068_f32, 2.4494898_f32, 2.6457512_f32, 2.828427_f32, 3_f32, 3.1622777_f32];
            coordinate_1 = (input_135471207182464[index(p) as usize] as f32);
            spline_index_2 = binary_search(density_function_Multiply_13_spline_coordinates_135471207185768, (coordinate_1 as f32));
            spline_result_f32_3 = advanced_hermite(density_function_Multiply_13_spline_coordinates_135471207185768, spline_values_0, density_function_Multiply_13_spline_derivatives_135471207185712, (coordinate_1 as f32), (spline_index_2 as i32));
            spline_result_f64_4 = (spline_result_f32_3 as f64);
            out[index(p)] = (spline_result_f64_4 * 0.8_f64);
        }
    }
    
    pub fn minecraft_realism_hills_slope_height_d5x1x5(out: &mut [f64; 25], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, input_135471207188352: &[f64; 25], input_135471207187056: &[f64; 25], input_135471207181728: &[f64; 25], input_135471207180040: &[f64; 25], input_135471207178464: &[f64; 25], host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        let mut spline_values_0: [f32; 1];
        let mut coordinate_1: f32;
        let mut spline_index_2: i32;
        let mut spline_values_3: [f32; 1];
        let mut coordinate_4: f32;
        let mut spline_index_5: i32;
        let mut spline_values_6: [f32; 2];
        let mut coordinate_7: f32;
        let mut spline_index_8: i32;
        let mut spline_result_f32_9: f32;
        let mut spline_result_f64_10: f64;
        let mut spline_values_11: [f32; 1];
        let mut coordinate_12: f32;
        let mut spline_index_13: i32;
        let mut spline_values_14: [f32; 1];
        let mut coordinate_15: f32;
        let mut spline_index_16: i32;
        let mut spline_values_17: [f32; 2];
        let mut coordinate_18: f32;
        let mut spline_index_19: i32;
        let mut spline_result_f32_20: f32;
        let mut spline_result_f64_21: f64;
        for p in iter_3d(5_i32, 1_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            spline_values_0 = [0_f32];
            coordinate_1 = (input_135471207187056[index(p) as usize] as f32);
            spline_index_2 = binary_search(minecraft_realism_hills_slope_height_spline_coordinates_135471207188240, (coordinate_1 as f32));
            spline_values_3 = [0_f32];
            coordinate_4 = (input_135471207181728[index(p) as usize] as f32);
            spline_index_5 = binary_search(minecraft_realism_hills_slope_height_spline_coordinates_135471207186944, (coordinate_4 as f32));
            spline_values_6 = [advanced_hermite(minecraft_realism_hills_slope_height_spline_coordinates_135471207188240, spline_values_0, minecraft_realism_hills_slope_height_spline_derivatives_135471207188184, (coordinate_1 as f32), (spline_index_2 as i32)), advanced_hermite(minecraft_realism_hills_slope_height_spline_coordinates_135471207186944, spline_values_3, minecraft_realism_hills_slope_height_spline_derivatives_135471207186888, (coordinate_4 as f32), (spline_index_5 as i32))];
            coordinate_7 = (input_135471207187056[index(p) as usize] as f32);
            spline_index_8 = binary_search(minecraft_realism_hills_slope_height_spline_coordinates_135471207181616, (coordinate_7 as f32));
            spline_result_f32_9 = advanced_hermite(minecraft_realism_hills_slope_height_spline_coordinates_135471207181616, spline_values_6, minecraft_realism_hills_slope_height_spline_derivatives_135471207181560, (coordinate_7 as f32), (spline_index_8 as i32));
            spline_result_f64_10 = (spline_result_f32_9 as f64);
            spline_values_11 = [0_f32];
            coordinate_12 = (input_135471207180040[index(p) as usize] as f32);
            spline_index_13 = binary_search(minecraft_realism_hills_slope_height_spline_coordinates_135471207181224, (coordinate_12 as f32));
            spline_values_14 = [0_f32];
            coordinate_15 = (input_135471207180040[index(p) as usize] as f32);
            spline_index_16 = binary_search(minecraft_realism_hills_slope_height_spline_coordinates_135471207179928, (coordinate_15 as f32));
            spline_values_17 = [advanced_hermite(minecraft_realism_hills_slope_height_spline_coordinates_135471207181224, spline_values_11, minecraft_realism_hills_slope_height_spline_derivatives_135471207181168, (coordinate_12 as f32), (spline_index_13 as i32)), advanced_hermite(minecraft_realism_hills_slope_height_spline_coordinates_135471207179928, spline_values_14, minecraft_realism_hills_slope_height_spline_derivatives_135471207179872, (coordinate_15 as f32), (spline_index_16 as i32))];
            coordinate_18 = (input_135471207178464[index(p) as usize] as f32);
            spline_index_19 = binary_search(minecraft_realism_hills_slope_height_spline_coordinates_135471207179648, (coordinate_18 as f32));
            spline_result_f32_20 = advanced_hermite(minecraft_realism_hills_slope_height_spline_coordinates_135471207179648, spline_values_17, minecraft_realism_hills_slope_height_spline_derivatives_135471207179592, (coordinate_18 as f32), (spline_index_19 as i32));
            spline_result_f64_21 = (spline_result_f32_20 as f64);
            out[index(p)] = ((0_f64 + ((input_135471207188352[flat_y_zero_index(pos3, 5_i32, 5_i32) as usize] * 0.25_f64) + -0.5_f64)) + ((spline_result_f64_10 * 0.2_f64) + spline_result_f64_21));
        }
    }
    
    pub fn density_function_ShiftedNoise_33_d5x1x5(out: &mut [f64; 25], origin: Vec3, origin_scale: Vec3, position_scale: Vec3, perm_table_minecraft_gravel_0_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__8: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__7: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__6: &PerlinNoiseSampler, perm_table_minecraft_gravel_0_octave__5: &PerlinNoiseSampler, perm_table_minecraft_gravel_1_octave__5: &PerlinNoiseSampler, host_consts: &HostConsts) {
        let mut rpos3: Vec3;
        for p in iter_3d(5_i32, 1_i32, 5_i32) {
            let pos3 = p.pos;
            rpos3 = ((origin * origin_scale) + (pos3 * position_scale));
            out[index(p)] = density_function_ShiftedNoise_33_d5x1x5_minecraft_gravel_shifted_34(((rpos3 * Vec3::new(0.12_f64, 0_f64, 0.12_f64)) + Vec3::new(1_f64, 0_f64, 0_f64)), perm_table_minecraft_gravel_0_octave__8, perm_table_minecraft_gravel_1_octave__8, perm_table_minecraft_gravel_0_octave__7, perm_table_minecraft_gravel_1_octave__7, perm_table_minecraft_gravel_0_octave__6, perm_table_minecraft_gravel_1_octave__6, perm_table_minecraft_gravel_0_octave__5, perm_table_minecraft_gravel_1_octave__5, host_consts);
        }
    }
    
}


pub mod orchestration {
    // Auto-generated inline Rust module from RCL
    #[allow(dead_code)]
    #[allow(unused_variables)]
    #[allow(unused_imports)]
    #[allow(warnings)]
    #[allow(clippy::all)]
    
    use super::*;
    use super::density_function::*;
    
    pub struct OrchestrationOutput {
        pub final_density: Box<[f64; 536576]>,
        pub final_density_with_beardify: Box<[f64; 536576]>,
    }
    
    pub struct PermutationTables {
        pub minecraft_jagged_0_octave__16: Box<PerlinNoiseSampler>,
        pub minecraft_jagged_1_octave__16: Box<PerlinNoiseSampler>,
        pub minecraft_jagged_0_octave__15: Box<PerlinNoiseSampler>,
        pub minecraft_jagged_1_octave__15: Box<PerlinNoiseSampler>,
        pub minecraft_jagged_0_octave__14: Box<PerlinNoiseSampler>,
        pub minecraft_jagged_1_octave__14: Box<PerlinNoiseSampler>,
        pub minecraft_jagged_0_octave__13: Box<PerlinNoiseSampler>,
        pub minecraft_jagged_1_octave__13: Box<PerlinNoiseSampler>,
        pub minecraft_jagged_0_octave__12: Box<PerlinNoiseSampler>,
        pub minecraft_jagged_1_octave__12: Box<PerlinNoiseSampler>,
        pub minecraft_jagged_0_octave__11: Box<PerlinNoiseSampler>,
        pub minecraft_jagged_1_octave__11: Box<PerlinNoiseSampler>,
        pub minecraft_jagged_0_octave__10: Box<PerlinNoiseSampler>,
        pub minecraft_jagged_1_octave__10: Box<PerlinNoiseSampler>,
        pub minecraft_jagged_0_octave__9: Box<PerlinNoiseSampler>,
        pub minecraft_jagged_1_octave__9: Box<PerlinNoiseSampler>,
        pub minecraft_jagged_0_octave__8: Box<PerlinNoiseSampler>,
        pub minecraft_jagged_1_octave__8: Box<PerlinNoiseSampler>,
        pub minecraft_jagged_0_octave__7: Box<PerlinNoiseSampler>,
        pub minecraft_jagged_1_octave__7: Box<PerlinNoiseSampler>,
        pub minecraft_jagged_0_octave__6: Box<PerlinNoiseSampler>,
        pub minecraft_jagged_1_octave__6: Box<PerlinNoiseSampler>,
        pub minecraft_jagged_0_octave__5: Box<PerlinNoiseSampler>,
        pub minecraft_jagged_1_octave__5: Box<PerlinNoiseSampler>,
        pub minecraft_jagged_0_octave__4: Box<PerlinNoiseSampler>,
        pub minecraft_jagged_1_octave__4: Box<PerlinNoiseSampler>,
        pub minecraft_jagged_0_octave__3: Box<PerlinNoiseSampler>,
        pub minecraft_jagged_1_octave__3: Box<PerlinNoiseSampler>,
        pub minecraft_jagged_0_octave__2: Box<PerlinNoiseSampler>,
        pub minecraft_jagged_1_octave__2: Box<PerlinNoiseSampler>,
        pub minecraft_jagged_0_octave__1: Box<PerlinNoiseSampler>,
        pub minecraft_jagged_1_octave__1: Box<PerlinNoiseSampler>,
        pub minecraft_gravel_0_octave__8: Box<PerlinNoiseSampler>,
        pub minecraft_gravel_1_octave__8: Box<PerlinNoiseSampler>,
        pub minecraft_gravel_0_octave__7: Box<PerlinNoiseSampler>,
        pub minecraft_gravel_1_octave__7: Box<PerlinNoiseSampler>,
        pub minecraft_gravel_0_octave__6: Box<PerlinNoiseSampler>,
        pub minecraft_gravel_1_octave__6: Box<PerlinNoiseSampler>,
        pub minecraft_gravel_0_octave__5: Box<PerlinNoiseSampler>,
        pub minecraft_gravel_1_octave__5: Box<PerlinNoiseSampler>,
        pub minecraft_cave_layer_0_octave__8: Box<PerlinNoiseSampler>,
        pub minecraft_cave_layer_1_octave__8: Box<PerlinNoiseSampler>,
    }
    
    pub fn orchestration(origin: Vec3, perm_tables: &PermutationTables, host_consts: &HostConsts) -> OrchestrationOutput {
        let mut density_function_YClampedGradient_76_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_0_0: () = density_function_YClampedGradient_76_d5x263x5((&mut density_function_YClampedGradient_76_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), host_consts);
        let mut minecraft_jagged_21_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_1: () = minecraft_jagged_21_d5x1x5((&mut minecraft_jagged_21_d5x1x5_output), origin, Vec3::new(300_f64, 0_f64, 300_f64), Vec3::new(1200_f64, 0_f64, 1200_f64), (&perm_tables.minecraft_jagged_0_octave__16), (&perm_tables.minecraft_jagged_1_octave__16), (&perm_tables.minecraft_jagged_0_octave__15), (&perm_tables.minecraft_jagged_1_octave__15), (&perm_tables.minecraft_jagged_0_octave__14), (&perm_tables.minecraft_jagged_1_octave__14), (&perm_tables.minecraft_jagged_0_octave__13), (&perm_tables.minecraft_jagged_1_octave__13), (&perm_tables.minecraft_jagged_0_octave__12), (&perm_tables.minecraft_jagged_1_octave__12), (&perm_tables.minecraft_jagged_0_octave__11), (&perm_tables.minecraft_jagged_1_octave__11), (&perm_tables.minecraft_jagged_0_octave__10), (&perm_tables.minecraft_jagged_1_octave__10), (&perm_tables.minecraft_jagged_0_octave__9), (&perm_tables.minecraft_jagged_1_octave__9), (&perm_tables.minecraft_jagged_0_octave__8), (&perm_tables.minecraft_jagged_1_octave__8), (&perm_tables.minecraft_jagged_0_octave__7), (&perm_tables.minecraft_jagged_1_octave__7), (&perm_tables.minecraft_jagged_0_octave__6), (&perm_tables.minecraft_jagged_1_octave__6), (&perm_tables.minecraft_jagged_0_octave__5), (&perm_tables.minecraft_jagged_1_octave__5), (&perm_tables.minecraft_jagged_0_octave__4), (&perm_tables.minecraft_jagged_1_octave__4), (&perm_tables.minecraft_jagged_0_octave__3), (&perm_tables.minecraft_jagged_1_octave__3), (&perm_tables.minecraft_jagged_0_octave__2), (&perm_tables.minecraft_jagged_1_octave__2), (&perm_tables.minecraft_jagged_0_octave__1), (&perm_tables.minecraft_jagged_1_octave__1), host_consts);
        let mut minecraft_jagged_5_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_2: () = minecraft_jagged_5_d5x1x5((&mut minecraft_jagged_5_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&perm_tables.minecraft_jagged_0_octave__16), (&perm_tables.minecraft_jagged_1_octave__16), (&perm_tables.minecraft_jagged_0_octave__15), (&perm_tables.minecraft_jagged_1_octave__15), (&perm_tables.minecraft_jagged_0_octave__14), (&perm_tables.minecraft_jagged_1_octave__14), (&perm_tables.minecraft_jagged_0_octave__13), (&perm_tables.minecraft_jagged_1_octave__13), (&perm_tables.minecraft_jagged_0_octave__12), (&perm_tables.minecraft_jagged_1_octave__12), (&perm_tables.minecraft_jagged_0_octave__11), (&perm_tables.minecraft_jagged_1_octave__11), (&perm_tables.minecraft_jagged_0_octave__10), (&perm_tables.minecraft_jagged_1_octave__10), (&perm_tables.minecraft_jagged_0_octave__9), (&perm_tables.minecraft_jagged_1_octave__9), (&perm_tables.minecraft_jagged_0_octave__8), (&perm_tables.minecraft_jagged_1_octave__8), (&perm_tables.minecraft_jagged_0_octave__7), (&perm_tables.minecraft_jagged_1_octave__7), (&perm_tables.minecraft_jagged_0_octave__6), (&perm_tables.minecraft_jagged_1_octave__6), (&perm_tables.minecraft_jagged_0_octave__5), (&perm_tables.minecraft_jagged_1_octave__5), (&perm_tables.minecraft_jagged_0_octave__4), (&perm_tables.minecraft_jagged_1_octave__4), (&perm_tables.minecraft_jagged_0_octave__3), (&perm_tables.minecraft_jagged_1_octave__3), (&perm_tables.minecraft_jagged_0_octave__2), (&perm_tables.minecraft_jagged_1_octave__2), (&perm_tables.minecraft_jagged_0_octave__1), (&perm_tables.minecraft_jagged_1_octave__1), host_consts);
        let mut minecraft_jagged_39_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_3: () = minecraft_jagged_39_d5x1x5((&mut minecraft_jagged_39_d5x1x5_output), origin, Vec3::new(2_f64, 0_f64, 2_f64), Vec3::new(8_f64, 0_f64, 8_f64), (&perm_tables.minecraft_jagged_0_octave__16), (&perm_tables.minecraft_jagged_1_octave__16), (&perm_tables.minecraft_jagged_0_octave__15), (&perm_tables.minecraft_jagged_1_octave__15), (&perm_tables.minecraft_jagged_0_octave__14), (&perm_tables.minecraft_jagged_1_octave__14), (&perm_tables.minecraft_jagged_0_octave__13), (&perm_tables.minecraft_jagged_1_octave__13), (&perm_tables.minecraft_jagged_0_octave__12), (&perm_tables.minecraft_jagged_1_octave__12), (&perm_tables.minecraft_jagged_0_octave__11), (&perm_tables.minecraft_jagged_1_octave__11), (&perm_tables.minecraft_jagged_0_octave__10), (&perm_tables.minecraft_jagged_1_octave__10), (&perm_tables.minecraft_jagged_0_octave__9), (&perm_tables.minecraft_jagged_1_octave__9), (&perm_tables.minecraft_jagged_0_octave__8), (&perm_tables.minecraft_jagged_1_octave__8), (&perm_tables.minecraft_jagged_0_octave__7), (&perm_tables.minecraft_jagged_1_octave__7), (&perm_tables.minecraft_jagged_0_octave__6), (&perm_tables.minecraft_jagged_1_octave__6), (&perm_tables.minecraft_jagged_0_octave__5), (&perm_tables.minecraft_jagged_1_octave__5), (&perm_tables.minecraft_jagged_0_octave__4), (&perm_tables.minecraft_jagged_1_octave__4), (&perm_tables.minecraft_jagged_0_octave__3), (&perm_tables.minecraft_jagged_1_octave__3), (&perm_tables.minecraft_jagged_0_octave__2), (&perm_tables.minecraft_jagged_1_octave__2), (&perm_tables.minecraft_jagged_0_octave__1), (&perm_tables.minecraft_jagged_1_octave__1), host_consts);
        let mut minecraft_gravel_2_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_4: () = minecraft_gravel_2_d5x1x5((&mut minecraft_gravel_2_d5x1x5_output), origin, Vec3::new(0.05000000074505806_f64, 0_f64, 0.05000000074505806_f64), Vec3::new(0.20000000298023224_f64, 0_f64, 0.20000000298023224_f64), (&perm_tables.minecraft_gravel_0_octave__8), (&perm_tables.minecraft_gravel_1_octave__8), (&perm_tables.minecraft_gravel_0_octave__7), (&perm_tables.minecraft_gravel_1_octave__7), (&perm_tables.minecraft_gravel_0_octave__6), (&perm_tables.minecraft_gravel_1_octave__6), (&perm_tables.minecraft_gravel_0_octave__5), (&perm_tables.minecraft_gravel_1_octave__5), host_consts);
        let mut minecraft_cave_layer_22_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_5: () = minecraft_cave_layer_22_d5x1x5((&mut minecraft_cave_layer_22_d5x1x5_output), origin, Vec3::new(0.30000001192092896_f64, 0_f64, 0.30000001192092896_f64), Vec3::new(1.2000000476837158_f64, 0_f64, 1.2000000476837158_f64), (&perm_tables.minecraft_cave_layer_0_octave__8), (&perm_tables.minecraft_cave_layer_1_octave__8), host_consts);
        let mut minecraft_cave_layer_1_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_6: () = minecraft_cave_layer_1_d5x1x5((&mut minecraft_cave_layer_1_d5x1x5_output), origin, Vec3::new(0.20000000298023224_f64, 0_f64, 0.20000000298023224_f64), Vec3::new(0.800000011920929_f64, 0_f64, 0.800000011920929_f64), (&perm_tables.minecraft_cave_layer_0_octave__8), (&perm_tables.minecraft_cave_layer_1_octave__8), host_consts);
        let mut minecraft_gravel_31_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_7: () = minecraft_gravel_31_d5x1x5((&mut minecraft_gravel_31_d5x1x5_output), origin, Vec3::new(0.11999999731779099_f64, 0_f64, 0.11999999731779099_f64), Vec3::new(0.47999998927116394_f64, 0_f64, 0.47999998927116394_f64), (&perm_tables.minecraft_gravel_0_octave__8), (&perm_tables.minecraft_gravel_1_octave__8), (&perm_tables.minecraft_gravel_0_octave__7), (&perm_tables.minecraft_gravel_1_octave__7), (&perm_tables.minecraft_gravel_0_octave__6), (&perm_tables.minecraft_gravel_1_octave__6), (&perm_tables.minecraft_gravel_0_octave__5), (&perm_tables.minecraft_gravel_1_octave__5), host_consts);
        let mut minecraft_cave_layer_7_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_8: () = minecraft_cave_layer_7_d5x1x5((&mut minecraft_cave_layer_7_d5x1x5_output), origin, Vec3::new(0.10000000149011612_f64, 0_f64, 0.10000000149011612_f64), Vec3::new(0.4000000059604645_f64, 0_f64, 0.4000000059604645_f64), (&perm_tables.minecraft_cave_layer_0_octave__8), (&perm_tables.minecraft_cave_layer_1_octave__8), host_consts);
        let mut density_function_ShiftedNoise_63_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_0_9: () = density_function_ShiftedNoise_63_d5x263x5((&mut density_function_ShiftedNoise_63_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&perm_tables.minecraft_gravel_0_octave__8), (&perm_tables.minecraft_gravel_1_octave__8), (&perm_tables.minecraft_gravel_0_octave__7), (&perm_tables.minecraft_gravel_1_octave__7), (&perm_tables.minecraft_gravel_0_octave__6), (&perm_tables.minecraft_gravel_1_octave__6), (&perm_tables.minecraft_gravel_0_octave__5), (&perm_tables.minecraft_gravel_1_octave__5), host_consts);
        let mut minecraft_cave_layer_71_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_0_10: () = minecraft_cave_layer_71_d5x263x5((&mut minecraft_cave_layer_71_d5x263x5_output), origin, Vec3::new(10_f64, 4_f64, 10_f64), Vec3::new(40_f64, 32_f64, 40_f64), (&perm_tables.minecraft_cave_layer_0_octave__8), (&perm_tables.minecraft_cave_layer_1_octave__8), host_consts);
        let mut minecraft_cave_layer_10_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_11: () = minecraft_cave_layer_10_d5x1x5((&mut minecraft_cave_layer_10_d5x1x5_output), origin, Vec3::new(0.15000000596046448_f64, 0_f64, 0.15000000596046448_f64), Vec3::new(0.6000000238418579_f64, 0_f64, 0.6000000238418579_f64), (&perm_tables.minecraft_cave_layer_0_octave__8), (&perm_tables.minecraft_cave_layer_1_octave__8), host_consts);
        let mut minecraft_cave_layer_69_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_0_12: () = minecraft_cave_layer_69_d5x263x5((&mut minecraft_cave_layer_69_d5x263x5_output), origin, Vec3::new(4_f64, 4_f64, 4_f64), Vec3::new(16_f64, 32_f64, 16_f64), (&perm_tables.minecraft_cave_layer_0_octave__8), (&perm_tables.minecraft_cave_layer_1_octave__8), host_consts);
        let mut density_function_ShiftedNoise_61_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_0_13: () = density_function_ShiftedNoise_61_d5x263x5((&mut density_function_ShiftedNoise_61_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&perm_tables.minecraft_gravel_0_octave__8), (&perm_tables.minecraft_gravel_1_octave__8), (&perm_tables.minecraft_gravel_0_octave__7), (&perm_tables.minecraft_gravel_1_octave__7), (&perm_tables.minecraft_gravel_0_octave__6), (&perm_tables.minecraft_gravel_1_octave__6), (&perm_tables.minecraft_gravel_0_octave__5), (&perm_tables.minecraft_gravel_1_octave__5), host_consts);
        let mut minecraft_jagged_37_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_14: () = minecraft_jagged_37_d5x1x5((&mut minecraft_jagged_37_d5x1x5_output), origin, Vec3::new(600_f64, 0_f64, 600_f64), Vec3::new(2400_f64, 0_f64, 2400_f64), (&perm_tables.minecraft_jagged_0_octave__16), (&perm_tables.minecraft_jagged_1_octave__16), (&perm_tables.minecraft_jagged_0_octave__15), (&perm_tables.minecraft_jagged_1_octave__15), (&perm_tables.minecraft_jagged_0_octave__14), (&perm_tables.minecraft_jagged_1_octave__14), (&perm_tables.minecraft_jagged_0_octave__13), (&perm_tables.minecraft_jagged_1_octave__13), (&perm_tables.minecraft_jagged_0_octave__12), (&perm_tables.minecraft_jagged_1_octave__12), (&perm_tables.minecraft_jagged_0_octave__11), (&perm_tables.minecraft_jagged_1_octave__11), (&perm_tables.minecraft_jagged_0_octave__10), (&perm_tables.minecraft_jagged_1_octave__10), (&perm_tables.minecraft_jagged_0_octave__9), (&perm_tables.minecraft_jagged_1_octave__9), (&perm_tables.minecraft_jagged_0_octave__8), (&perm_tables.minecraft_jagged_1_octave__8), (&perm_tables.minecraft_jagged_0_octave__7), (&perm_tables.minecraft_jagged_1_octave__7), (&perm_tables.minecraft_jagged_0_octave__6), (&perm_tables.minecraft_jagged_1_octave__6), (&perm_tables.minecraft_jagged_0_octave__5), (&perm_tables.minecraft_jagged_1_octave__5), (&perm_tables.minecraft_jagged_0_octave__4), (&perm_tables.minecraft_jagged_1_octave__4), (&perm_tables.minecraft_jagged_0_octave__3), (&perm_tables.minecraft_jagged_1_octave__3), (&perm_tables.minecraft_jagged_0_octave__2), (&perm_tables.minecraft_jagged_1_octave__2), (&perm_tables.minecraft_jagged_0_octave__1), (&perm_tables.minecraft_jagged_1_octave__1), host_consts);
        let mut Beardifier_d16x2096x16_output: Box<[f64; 536576]> = make_buffer::<536576>();
        let _shader_call_0_15: () = Beardifier_d16x2096x16((&mut Beardifier_d16x2096x16_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(1_f64, 1_f64, 1_f64), host_consts);
        let mut minecraft_gravel_19_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_16: () = minecraft_gravel_19_d5x1x5((&mut minecraft_gravel_19_d5x1x5_output), origin, Vec3::new(0.20000000298023224_f64, 0_f64, 0.20000000298023224_f64), Vec3::new(0.800000011920929_f64, 0_f64, 0.800000011920929_f64), (&perm_tables.minecraft_gravel_0_octave__8), (&perm_tables.minecraft_gravel_1_octave__8), (&perm_tables.minecraft_gravel_0_octave__7), (&perm_tables.minecraft_gravel_1_octave__7), (&perm_tables.minecraft_gravel_0_octave__6), (&perm_tables.minecraft_gravel_1_octave__6), (&perm_tables.minecraft_gravel_0_octave__5), (&perm_tables.minecraft_gravel_1_octave__5), host_consts);
        let mut minecraft_jagged_3_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_17: () = minecraft_jagged_3_d5x1x5((&mut minecraft_jagged_3_d5x1x5_output), origin, Vec3::new(175_f64, 0_f64, 175_f64), Vec3::new(700_f64, 0_f64, 700_f64), (&perm_tables.minecraft_jagged_0_octave__16), (&perm_tables.minecraft_jagged_1_octave__16), (&perm_tables.minecraft_jagged_0_octave__15), (&perm_tables.minecraft_jagged_1_octave__15), (&perm_tables.minecraft_jagged_0_octave__14), (&perm_tables.minecraft_jagged_1_octave__14), (&perm_tables.minecraft_jagged_0_octave__13), (&perm_tables.minecraft_jagged_1_octave__13), (&perm_tables.minecraft_jagged_0_octave__12), (&perm_tables.minecraft_jagged_1_octave__12), (&perm_tables.minecraft_jagged_0_octave__11), (&perm_tables.minecraft_jagged_1_octave__11), (&perm_tables.minecraft_jagged_0_octave__10), (&perm_tables.minecraft_jagged_1_octave__10), (&perm_tables.minecraft_jagged_0_octave__9), (&perm_tables.minecraft_jagged_1_octave__9), (&perm_tables.minecraft_jagged_0_octave__8), (&perm_tables.minecraft_jagged_1_octave__8), (&perm_tables.minecraft_jagged_0_octave__7), (&perm_tables.minecraft_jagged_1_octave__7), (&perm_tables.minecraft_jagged_0_octave__6), (&perm_tables.minecraft_jagged_1_octave__6), (&perm_tables.minecraft_jagged_0_octave__5), (&perm_tables.minecraft_jagged_1_octave__5), (&perm_tables.minecraft_jagged_0_octave__4), (&perm_tables.minecraft_jagged_1_octave__4), (&perm_tables.minecraft_jagged_0_octave__3), (&perm_tables.minecraft_jagged_1_octave__3), (&perm_tables.minecraft_jagged_0_octave__2), (&perm_tables.minecraft_jagged_1_octave__2), (&perm_tables.minecraft_jagged_0_octave__1), (&perm_tables.minecraft_jagged_1_octave__1), host_consts);
        let mut density_function_ShiftedNoise_33_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_18: () = density_function_ShiftedNoise_33_d5x1x5((&mut density_function_ShiftedNoise_33_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&perm_tables.minecraft_gravel_0_octave__8), (&perm_tables.minecraft_gravel_1_octave__8), (&perm_tables.minecraft_gravel_0_octave__7), (&perm_tables.minecraft_gravel_1_octave__7), (&perm_tables.minecraft_gravel_0_octave__6), (&perm_tables.minecraft_gravel_1_octave__6), (&perm_tables.minecraft_gravel_0_octave__5), (&perm_tables.minecraft_gravel_1_octave__5), host_consts);
        let mut minecraft_cave_layer_68_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_0_19: () = minecraft_cave_layer_68_d5x263x5((&mut minecraft_cave_layer_68_d5x263x5_output), origin, Vec3::new(6_f64, 6_f64, 6_f64), Vec3::new(24_f64, 48_f64, 24_f64), (&perm_tables.minecraft_cave_layer_0_octave__8), (&perm_tables.minecraft_cave_layer_1_octave__8), host_consts);
        let mut minecraft_jagged_18_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_20: () = minecraft_jagged_18_d5x1x5((&mut minecraft_jagged_18_d5x1x5_output), origin, Vec3::new(250_f64, 0_f64, 250_f64), Vec3::new(1000_f64, 0_f64, 1000_f64), (&perm_tables.minecraft_jagged_0_octave__16), (&perm_tables.minecraft_jagged_1_octave__16), (&perm_tables.minecraft_jagged_0_octave__15), (&perm_tables.minecraft_jagged_1_octave__15), (&perm_tables.minecraft_jagged_0_octave__14), (&perm_tables.minecraft_jagged_1_octave__14), (&perm_tables.minecraft_jagged_0_octave__13), (&perm_tables.minecraft_jagged_1_octave__13), (&perm_tables.minecraft_jagged_0_octave__12), (&perm_tables.minecraft_jagged_1_octave__12), (&perm_tables.minecraft_jagged_0_octave__11), (&perm_tables.minecraft_jagged_1_octave__11), (&perm_tables.minecraft_jagged_0_octave__10), (&perm_tables.minecraft_jagged_1_octave__10), (&perm_tables.minecraft_jagged_0_octave__9), (&perm_tables.minecraft_jagged_1_octave__9), (&perm_tables.minecraft_jagged_0_octave__8), (&perm_tables.minecraft_jagged_1_octave__8), (&perm_tables.minecraft_jagged_0_octave__7), (&perm_tables.minecraft_jagged_1_octave__7), (&perm_tables.minecraft_jagged_0_octave__6), (&perm_tables.minecraft_jagged_1_octave__6), (&perm_tables.minecraft_jagged_0_octave__5), (&perm_tables.minecraft_jagged_1_octave__5), (&perm_tables.minecraft_jagged_0_octave__4), (&perm_tables.minecraft_jagged_1_octave__4), (&perm_tables.minecraft_jagged_0_octave__3), (&perm_tables.minecraft_jagged_1_octave__3), (&perm_tables.minecraft_jagged_0_octave__2), (&perm_tables.minecraft_jagged_1_octave__2), (&perm_tables.minecraft_jagged_0_octave__1), (&perm_tables.minecraft_jagged_1_octave__1), host_consts);
        let mut minecraft_cave_layer_73_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_0_21: () = minecraft_cave_layer_73_d5x263x5((&mut minecraft_cave_layer_73_d5x263x5_output), origin, Vec3::new(5_f64, 2_f64, 5_f64), Vec3::new(20_f64, 16_f64, 20_f64), (&perm_tables.minecraft_cave_layer_0_octave__8), (&perm_tables.minecraft_cave_layer_1_octave__8), host_consts);
        let mut minecraft_cave_layer_67_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_0_22: () = minecraft_cave_layer_67_d5x263x5((&mut minecraft_cave_layer_67_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&perm_tables.minecraft_cave_layer_0_octave__8), (&perm_tables.minecraft_cave_layer_1_octave__8), host_consts);
        let mut minecraft_gravel_8_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_23: () = minecraft_gravel_8_d5x1x5((&mut minecraft_gravel_8_d5x1x5_output), origin, Vec3::new(0.10000000149011612_f64, 0_f64, 0.10000000149011612_f64), Vec3::new(0.4000000059604645_f64, 0_f64, 0.4000000059604645_f64), (&perm_tables.minecraft_gravel_0_octave__8), (&perm_tables.minecraft_gravel_1_octave__8), (&perm_tables.minecraft_gravel_0_octave__7), (&perm_tables.minecraft_gravel_1_octave__7), (&perm_tables.minecraft_gravel_0_octave__6), (&perm_tables.minecraft_gravel_1_octave__6), (&perm_tables.minecraft_gravel_0_octave__5), (&perm_tables.minecraft_gravel_1_octave__5), host_consts);
        let mut minecraft_gravel_70_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_24: () = minecraft_gravel_70_d5x1x5((&mut minecraft_gravel_70_d5x1x5_output), origin, Vec3::new(0.029999999329447746_f64, 0_f64, 0.029999999329447746_f64), Vec3::new(0.11999999731779099_f64, 0_f64, 0.11999999731779099_f64), (&perm_tables.minecraft_gravel_0_octave__8), (&perm_tables.minecraft_gravel_1_octave__8), (&perm_tables.minecraft_gravel_0_octave__7), (&perm_tables.minecraft_gravel_1_octave__7), (&perm_tables.minecraft_gravel_0_octave__6), (&perm_tables.minecraft_gravel_1_octave__6), (&perm_tables.minecraft_gravel_0_octave__5), (&perm_tables.minecraft_gravel_1_octave__5), host_consts);
        let mut density_function_ShiftedNoise_27_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_25: () = density_function_ShiftedNoise_27_d5x1x5((&mut density_function_ShiftedNoise_27_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&perm_tables.minecraft_gravel_0_octave__8), (&perm_tables.minecraft_gravel_1_octave__8), (&perm_tables.minecraft_gravel_0_octave__7), (&perm_tables.minecraft_gravel_1_octave__7), (&perm_tables.minecraft_gravel_0_octave__6), (&perm_tables.minecraft_gravel_1_octave__6), (&perm_tables.minecraft_gravel_0_octave__5), (&perm_tables.minecraft_gravel_1_octave__5), host_consts);
        let mut density_function_ShiftedNoise_58_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_0_26: () = density_function_ShiftedNoise_58_d5x263x5((&mut density_function_ShiftedNoise_58_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&perm_tables.minecraft_gravel_0_octave__8), (&perm_tables.minecraft_gravel_1_octave__8), (&perm_tables.minecraft_gravel_0_octave__7), (&perm_tables.minecraft_gravel_1_octave__7), (&perm_tables.minecraft_gravel_0_octave__6), (&perm_tables.minecraft_gravel_1_octave__6), (&perm_tables.minecraft_gravel_0_octave__5), (&perm_tables.minecraft_gravel_1_octave__5), host_consts);
        let mut minecraft_gravel_23_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_27: () = minecraft_gravel_23_d5x1x5((&mut minecraft_gravel_23_d5x1x5_output), origin, Vec3::new(0.10999999940395355_f64, 0_f64, 0.10999999940395355_f64), Vec3::new(0.4399999976158142_f64, 0_f64, 0.4399999976158142_f64), (&perm_tables.minecraft_gravel_0_octave__8), (&perm_tables.minecraft_gravel_1_octave__8), (&perm_tables.minecraft_gravel_0_octave__7), (&perm_tables.minecraft_gravel_1_octave__7), (&perm_tables.minecraft_gravel_0_octave__6), (&perm_tables.minecraft_gravel_1_octave__6), (&perm_tables.minecraft_gravel_0_octave__5), (&perm_tables.minecraft_gravel_1_octave__5), host_consts);
        let mut density_function_ShiftedNoise_52_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_0_28: () = density_function_ShiftedNoise_52_d5x263x5((&mut density_function_ShiftedNoise_52_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&perm_tables.minecraft_gravel_0_octave__8), (&perm_tables.minecraft_gravel_1_octave__8), (&perm_tables.minecraft_gravel_0_octave__7), (&perm_tables.minecraft_gravel_1_octave__7), (&perm_tables.minecraft_gravel_0_octave__6), (&perm_tables.minecraft_gravel_1_octave__6), (&perm_tables.minecraft_gravel_0_octave__5), (&perm_tables.minecraft_gravel_1_octave__5), host_consts);
        let mut density_function_ShiftedNoise_50_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_0_29: () = density_function_ShiftedNoise_50_d5x263x5((&mut density_function_ShiftedNoise_50_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&perm_tables.minecraft_gravel_0_octave__8), (&perm_tables.minecraft_gravel_1_octave__8), (&perm_tables.minecraft_gravel_0_octave__7), (&perm_tables.minecraft_gravel_1_octave__7), (&perm_tables.minecraft_gravel_0_octave__6), (&perm_tables.minecraft_gravel_1_octave__6), (&perm_tables.minecraft_gravel_0_octave__5), (&perm_tables.minecraft_gravel_1_octave__5), host_consts);
        let mut minecraft_jagged_9_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_30: () = minecraft_jagged_9_d5x1x5((&mut minecraft_jagged_9_d5x1x5_output), origin, Vec3::new(500_f64, 0_f64, 500_f64), Vec3::new(2000_f64, 0_f64, 2000_f64), (&perm_tables.minecraft_jagged_0_octave__16), (&perm_tables.minecraft_jagged_1_octave__16), (&perm_tables.minecraft_jagged_0_octave__15), (&perm_tables.minecraft_jagged_1_octave__15), (&perm_tables.minecraft_jagged_0_octave__14), (&perm_tables.minecraft_jagged_1_octave__14), (&perm_tables.minecraft_jagged_0_octave__13), (&perm_tables.minecraft_jagged_1_octave__13), (&perm_tables.minecraft_jagged_0_octave__12), (&perm_tables.minecraft_jagged_1_octave__12), (&perm_tables.minecraft_jagged_0_octave__11), (&perm_tables.minecraft_jagged_1_octave__11), (&perm_tables.minecraft_jagged_0_octave__10), (&perm_tables.minecraft_jagged_1_octave__10), (&perm_tables.minecraft_jagged_0_octave__9), (&perm_tables.minecraft_jagged_1_octave__9), (&perm_tables.minecraft_jagged_0_octave__8), (&perm_tables.minecraft_jagged_1_octave__8), (&perm_tables.minecraft_jagged_0_octave__7), (&perm_tables.minecraft_jagged_1_octave__7), (&perm_tables.minecraft_jagged_0_octave__6), (&perm_tables.minecraft_jagged_1_octave__6), (&perm_tables.minecraft_jagged_0_octave__5), (&perm_tables.minecraft_jagged_1_octave__5), (&perm_tables.minecraft_jagged_0_octave__4), (&perm_tables.minecraft_jagged_1_octave__4), (&perm_tables.minecraft_jagged_0_octave__3), (&perm_tables.minecraft_jagged_1_octave__3), (&perm_tables.minecraft_jagged_0_octave__2), (&perm_tables.minecraft_jagged_1_octave__2), (&perm_tables.minecraft_jagged_0_octave__1), (&perm_tables.minecraft_jagged_1_octave__1), host_consts);
        let mut minecraft_cave_layer_75_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_0_31: () = minecraft_cave_layer_75_d5x263x5((&mut minecraft_cave_layer_75_d5x263x5_output), origin, Vec3::new(0.5_f64, 0.5_f64, 0.5_f64), Vec3::new(2_f64, 4_f64, 2_f64), (&perm_tables.minecraft_cave_layer_0_octave__8), (&perm_tables.minecraft_cave_layer_1_octave__8), host_consts);
        let mut minecraft_cave_layer_30_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_32: () = minecraft_cave_layer_30_d5x1x5((&mut minecraft_cave_layer_30_d5x1x5_output), origin, Vec3::new(0.25_f64, 0_f64, 0.25_f64), Vec3::new(1_f64, 0_f64, 1_f64), (&perm_tables.minecraft_cave_layer_0_octave__8), (&perm_tables.minecraft_cave_layer_1_octave__8), host_consts);
        let mut minecraft_gravel_11_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_33: () = minecraft_gravel_11_d5x1x5((&mut minecraft_gravel_11_d5x1x5_output), origin, Vec3::new(0.25_f64, 0_f64, 0.25_f64), Vec3::new(1_f64, 0_f64, 1_f64), (&perm_tables.minecraft_gravel_0_octave__8), (&perm_tables.minecraft_gravel_1_octave__8), (&perm_tables.minecraft_gravel_0_octave__7), (&perm_tables.minecraft_gravel_1_octave__7), (&perm_tables.minecraft_gravel_0_octave__6), (&perm_tables.minecraft_gravel_1_octave__6), (&perm_tables.minecraft_gravel_0_octave__5), (&perm_tables.minecraft_gravel_1_octave__5), host_consts);
        let mut minecraft_jagged_29_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_34: () = minecraft_jagged_29_d5x1x5((&mut minecraft_jagged_29_d5x1x5_output), origin, Vec3::new(501_f64, 0_f64, 501_f64), Vec3::new(2004_f64, 0_f64, 2004_f64), (&perm_tables.minecraft_jagged_0_octave__16), (&perm_tables.minecraft_jagged_1_octave__16), (&perm_tables.minecraft_jagged_0_octave__15), (&perm_tables.minecraft_jagged_1_octave__15), (&perm_tables.minecraft_jagged_0_octave__14), (&perm_tables.minecraft_jagged_1_octave__14), (&perm_tables.minecraft_jagged_0_octave__13), (&perm_tables.minecraft_jagged_1_octave__13), (&perm_tables.minecraft_jagged_0_octave__12), (&perm_tables.minecraft_jagged_1_octave__12), (&perm_tables.minecraft_jagged_0_octave__11), (&perm_tables.minecraft_jagged_1_octave__11), (&perm_tables.minecraft_jagged_0_octave__10), (&perm_tables.minecraft_jagged_1_octave__10), (&perm_tables.minecraft_jagged_0_octave__9), (&perm_tables.minecraft_jagged_1_octave__9), (&perm_tables.minecraft_jagged_0_octave__8), (&perm_tables.minecraft_jagged_1_octave__8), (&perm_tables.minecraft_jagged_0_octave__7), (&perm_tables.minecraft_jagged_1_octave__7), (&perm_tables.minecraft_jagged_0_octave__6), (&perm_tables.minecraft_jagged_1_octave__6), (&perm_tables.minecraft_jagged_0_octave__5), (&perm_tables.minecraft_jagged_1_octave__5), (&perm_tables.minecraft_jagged_0_octave__4), (&perm_tables.minecraft_jagged_1_octave__4), (&perm_tables.minecraft_jagged_0_octave__3), (&perm_tables.minecraft_jagged_1_octave__3), (&perm_tables.minecraft_jagged_0_octave__2), (&perm_tables.minecraft_jagged_1_octave__2), (&perm_tables.minecraft_jagged_0_octave__1), (&perm_tables.minecraft_jagged_1_octave__1), host_consts);
        let mut density_function_ShiftedNoise_25_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_35: () = density_function_ShiftedNoise_25_d5x1x5((&mut density_function_ShiftedNoise_25_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&perm_tables.minecraft_gravel_0_octave__8), (&perm_tables.minecraft_gravel_1_octave__8), (&perm_tables.minecraft_gravel_0_octave__7), (&perm_tables.minecraft_gravel_1_octave__7), (&perm_tables.minecraft_gravel_0_octave__6), (&perm_tables.minecraft_gravel_1_octave__6), (&perm_tables.minecraft_gravel_0_octave__5), (&perm_tables.minecraft_gravel_1_octave__5), host_consts);
        let mut density_function_ShiftedNoise_16_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_36: () = density_function_ShiftedNoise_16_d5x1x5((&mut density_function_ShiftedNoise_16_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&perm_tables.minecraft_gravel_0_octave__8), (&perm_tables.minecraft_gravel_1_octave__8), (&perm_tables.minecraft_gravel_0_octave__7), (&perm_tables.minecraft_gravel_1_octave__7), (&perm_tables.minecraft_gravel_0_octave__6), (&perm_tables.minecraft_gravel_1_octave__6), (&perm_tables.minecraft_gravel_0_octave__5), (&perm_tables.minecraft_gravel_1_octave__5), host_consts);
        let mut minecraft_cave_layer_74_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_0_37: () = minecraft_cave_layer_74_d5x263x5((&mut minecraft_cave_layer_74_d5x263x5_output), origin, Vec3::new(2.5_f64, 1_f64, 2.5_f64), Vec3::new(10_f64, 8_f64, 10_f64), (&perm_tables.minecraft_cave_layer_0_octave__8), (&perm_tables.minecraft_cave_layer_1_octave__8), host_consts);
        let mut density_function_ShiftedNoise_35_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_38: () = density_function_ShiftedNoise_35_d5x1x5((&mut density_function_ShiftedNoise_35_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&perm_tables.minecraft_gravel_0_octave__8), (&perm_tables.minecraft_gravel_1_octave__8), (&perm_tables.minecraft_gravel_0_octave__7), (&perm_tables.minecraft_gravel_1_octave__7), (&perm_tables.minecraft_gravel_0_octave__6), (&perm_tables.minecraft_gravel_1_octave__6), (&perm_tables.minecraft_gravel_0_octave__5), (&perm_tables.minecraft_gravel_1_octave__5), host_consts);
        let mut density_function_ShiftedNoise_14_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_39: () = density_function_ShiftedNoise_14_d5x1x5((&mut density_function_ShiftedNoise_14_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&perm_tables.minecraft_gravel_0_octave__8), (&perm_tables.minecraft_gravel_1_octave__8), (&perm_tables.minecraft_gravel_0_octave__7), (&perm_tables.minecraft_gravel_1_octave__7), (&perm_tables.minecraft_gravel_0_octave__6), (&perm_tables.minecraft_gravel_1_octave__6), (&perm_tables.minecraft_gravel_0_octave__5), (&perm_tables.minecraft_gravel_1_octave__5), host_consts);
        let mut minecraft_jagged_72_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_40: () = minecraft_jagged_72_d5x1x5((&mut minecraft_jagged_72_d5x1x5_output), origin, Vec3::new(1000_f64, 0_f64, 1000_f64), Vec3::new(4000_f64, 0_f64, 4000_f64), (&perm_tables.minecraft_jagged_0_octave__16), (&perm_tables.minecraft_jagged_1_octave__16), (&perm_tables.minecraft_jagged_0_octave__15), (&perm_tables.minecraft_jagged_1_octave__15), (&perm_tables.minecraft_jagged_0_octave__14), (&perm_tables.minecraft_jagged_1_octave__14), (&perm_tables.minecraft_jagged_0_octave__13), (&perm_tables.minecraft_jagged_1_octave__13), (&perm_tables.minecraft_jagged_0_octave__12), (&perm_tables.minecraft_jagged_1_octave__12), (&perm_tables.minecraft_jagged_0_octave__11), (&perm_tables.minecraft_jagged_1_octave__11), (&perm_tables.minecraft_jagged_0_octave__10), (&perm_tables.minecraft_jagged_1_octave__10), (&perm_tables.minecraft_jagged_0_octave__9), (&perm_tables.minecraft_jagged_1_octave__9), (&perm_tables.minecraft_jagged_0_octave__8), (&perm_tables.minecraft_jagged_1_octave__8), (&perm_tables.minecraft_jagged_0_octave__7), (&perm_tables.minecraft_jagged_1_octave__7), (&perm_tables.minecraft_jagged_0_octave__6), (&perm_tables.minecraft_jagged_1_octave__6), (&perm_tables.minecraft_jagged_0_octave__5), (&perm_tables.minecraft_jagged_1_octave__5), (&perm_tables.minecraft_jagged_0_octave__4), (&perm_tables.minecraft_jagged_1_octave__4), (&perm_tables.minecraft_jagged_0_octave__3), (&perm_tables.minecraft_jagged_1_octave__3), (&perm_tables.minecraft_jagged_0_octave__2), (&perm_tables.minecraft_jagged_1_octave__2), (&perm_tables.minecraft_jagged_0_octave__1), (&perm_tables.minecraft_jagged_1_octave__1), host_consts);
        let mut minecraft_cave_layer_66_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_0_41: () = minecraft_cave_layer_66_d5x263x5((&mut minecraft_cave_layer_66_d5x263x5_output), origin, Vec3::new(8_f64, 8_f64, 8_f64), Vec3::new(32_f64, 64_f64, 32_f64), (&perm_tables.minecraft_cave_layer_0_octave__8), (&perm_tables.minecraft_cave_layer_1_octave__8), host_consts);
        let mut density_function_ShiftedNoise_56_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_0_42: () = density_function_ShiftedNoise_56_d5x263x5((&mut density_function_ShiftedNoise_56_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&perm_tables.minecraft_gravel_0_octave__8), (&perm_tables.minecraft_gravel_1_octave__8), (&perm_tables.minecraft_gravel_0_octave__7), (&perm_tables.minecraft_gravel_1_octave__7), (&perm_tables.minecraft_gravel_0_octave__6), (&perm_tables.minecraft_gravel_1_octave__6), (&perm_tables.minecraft_gravel_0_octave__5), (&perm_tables.minecraft_gravel_1_octave__5), host_consts);
        let mut density_function_Add_0_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_1_0: () = density_function_Add_0_d5x1x5((&mut density_function_Add_0_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&minecraft_cave_layer_1_d5x1x5_output), (&minecraft_gravel_2_d5x1x5_output), (&minecraft_jagged_3_d5x1x5_output), host_consts);
        let mut minecraft_realism_extreme_hills_jagged_original_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_1_1: () = minecraft_realism_extreme_hills_jagged_original_d5x1x5((&mut minecraft_realism_extreme_hills_jagged_original_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&minecraft_jagged_21_d5x1x5_output), host_consts);
        let mut minecraft_realism_extreme_hills_jagged_original_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_2: () = minecraft_realism_extreme_hills_jagged_original_d5x263x5((&mut minecraft_realism_extreme_hills_jagged_original_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_jagged_21_d5x1x5_output), host_consts);
        let mut minecraft_realism_extreme_hills_factor_original_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_1_3: () = minecraft_realism_extreme_hills_factor_original_d5x1x5((&mut minecraft_realism_extreme_hills_factor_original_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&minecraft_gravel_11_d5x1x5_output), host_consts);
        let mut minecraft_realism_hills_jagged_original_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_4: () = minecraft_realism_hills_jagged_original_d5x263x5((&mut minecraft_realism_hills_jagged_original_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_jagged_18_d5x1x5_output), host_consts);
        let mut minecraft_realism_extreme_hills_factor_original_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_5: () = minecraft_realism_extreme_hills_factor_original_d5x263x5((&mut minecraft_realism_extreme_hills_factor_original_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_gravel_11_d5x1x5_output), host_consts);
        let mut density_function_Add_46_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_6: () = density_function_Add_46_d5x263x5((&mut density_function_Add_46_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_cave_layer_1_d5x1x5_output), (&minecraft_gravel_2_d5x1x5_output), (&minecraft_jagged_3_d5x1x5_output), host_consts);
        let mut minecraft_caves_old_large_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_7: () = minecraft_caves_old_large_d5x263x5((&mut minecraft_caves_old_large_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_cave_layer_69_d5x263x5_output), host_consts);
        let mut minecraft_caves_barrier_2_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_8: () = minecraft_caves_barrier_2_d5x263x5((&mut minecraft_caves_barrier_2_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_cave_layer_67_d5x263x5_output), host_consts);
        let mut minecraft_realism_mountains_factor_original_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_1_9: () = minecraft_realism_mountains_factor_original_d5x1x5((&mut minecraft_realism_mountains_factor_original_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&minecraft_gravel_23_d5x1x5_output), host_consts);
        let mut density_function_Multiply_24_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_1_10: () = density_function_Multiply_24_d5x1x5((&mut density_function_Multiply_24_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&density_function_ShiftedNoise_25_d5x1x5_output), (&minecraft_gravel_23_d5x1x5_output), (&density_function_ShiftedNoise_27_d5x1x5_output), host_consts);
        let mut minecraft_realism_extreme_mountains_factor_original_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_11: () = minecraft_realism_extreme_mountains_factor_original_d5x263x5((&mut minecraft_realism_extreme_mountains_factor_original_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_gravel_31_d5x1x5_output), host_consts);
        let mut minecraft_realism_hills_jagged_original_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_1_12: () = minecraft_realism_hills_jagged_original_d5x1x5((&mut minecraft_realism_hills_jagged_original_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&minecraft_jagged_18_d5x1x5_output), host_consts);
        let mut minecraft_realism_extreme_mountains_factor_original_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_1_13: () = minecraft_realism_extreme_mountains_factor_original_d5x1x5((&mut minecraft_realism_extreme_mountains_factor_original_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&minecraft_gravel_31_d5x1x5_output), host_consts);
        let mut density_function_Add_45_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_14: () = density_function_Add_45_d5x263x5((&mut density_function_Add_45_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_cave_layer_7_d5x1x5_output), (&minecraft_gravel_8_d5x1x5_output), (&minecraft_jagged_9_d5x1x5_output), host_consts);
        let mut density_function_Abs_38_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_1_15: () = density_function_Abs_38_d5x1x5((&mut density_function_Abs_38_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&minecraft_jagged_39_d5x1x5_output), host_consts);
        let mut minecraft_realism_hills_factor_original_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_1_16: () = minecraft_realism_hills_factor_original_d5x1x5((&mut minecraft_realism_hills_factor_original_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&minecraft_gravel_19_d5x1x5_output), host_consts);
        let mut minecraft_caves_old_small_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_17: () = minecraft_caves_old_small_d5x263x5((&mut minecraft_caves_old_small_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_cave_layer_66_d5x263x5_output), host_consts);
        let mut minecraft_realism_extreme_hills_factor_divergence_squared_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_18: () = minecraft_realism_extreme_hills_factor_divergence_squared_d5x263x5((&mut minecraft_realism_extreme_hills_factor_divergence_squared_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&density_function_ShiftedNoise_50_d5x263x5_output), (&minecraft_gravel_11_d5x1x5_output), (&density_function_ShiftedNoise_52_d5x263x5_output), host_consts);
        let mut minecraft_realism_hills_factor_original_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_19: () = minecraft_realism_hills_factor_original_d5x263x5((&mut minecraft_realism_hills_factor_original_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_gravel_19_d5x1x5_output), host_consts);
        let mut minecraft_caves_old_medium_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_20: () = minecraft_caves_old_medium_d5x263x5((&mut minecraft_caves_old_medium_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_cave_layer_68_d5x263x5_output), host_consts);
        let mut minecraft_realism_mountains_jagged_original_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_21: () = minecraft_realism_mountains_jagged_original_d5x263x5((&mut minecraft_realism_mountains_jagged_original_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_jagged_29_d5x1x5_output), host_consts);
        let mut density_function_Abs_4_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_1_22: () = density_function_Abs_4_d5x1x5((&mut density_function_Abs_4_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&minecraft_jagged_5_d5x1x5_output), host_consts);
        let mut minecraft_realism_extreme_hills_factor_divergence_squared_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_1_23: () = minecraft_realism_extreme_hills_factor_divergence_squared_d5x1x5((&mut minecraft_realism_extreme_hills_factor_divergence_squared_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&density_function_ShiftedNoise_14_d5x1x5_output), (&minecraft_gravel_11_d5x1x5_output), (&density_function_ShiftedNoise_16_d5x1x5_output), host_consts);
        let mut minecraft_caves_barrier_1_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_24: () = minecraft_caves_barrier_1_d5x263x5((&mut minecraft_caves_barrier_1_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_cave_layer_75_d5x263x5_output), host_consts);
        let mut density_function_Abs_65_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_25: () = density_function_Abs_65_d5x263x5((&mut density_function_Abs_65_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_jagged_39_d5x1x5_output), host_consts);
        let mut minecraft_realism_extreme_mountains_jagged_original_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_26: () = minecraft_realism_extreme_mountains_jagged_original_d5x263x5((&mut minecraft_realism_extreme_mountains_jagged_original_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_jagged_37_d5x1x5_output), host_consts);
        let mut minecraft_realism_mountains_jagged_original_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_1_27: () = minecraft_realism_mountains_jagged_original_d5x1x5((&mut minecraft_realism_mountains_jagged_original_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&minecraft_jagged_29_d5x1x5_output), host_consts);
        let mut minecraft_determiner_cave_1_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_28: () = minecraft_determiner_cave_1_d5x263x5((&mut minecraft_determiner_cave_1_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_gravel_70_d5x1x5_output), host_consts);
        let mut minecraft_realism_extreme_mountains_jagged_original_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_1_29: () = minecraft_realism_extreme_mountains_jagged_original_d5x1x5((&mut minecraft_realism_extreme_mountains_jagged_original_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&minecraft_jagged_37_d5x1x5_output), host_consts);
        let mut density_function_Multiply_32_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_1_30: () = density_function_Multiply_32_d5x1x5((&mut density_function_Multiply_32_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&density_function_ShiftedNoise_33_d5x1x5_output), (&minecraft_gravel_31_d5x1x5_output), (&density_function_ShiftedNoise_35_d5x1x5_output), host_consts);
        let mut density_function_Add_6_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_1_31: () = density_function_Add_6_d5x1x5((&mut density_function_Add_6_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&minecraft_cave_layer_7_d5x1x5_output), (&minecraft_gravel_8_d5x1x5_output), (&minecraft_jagged_9_d5x1x5_output), host_consts);
        let mut density_function_Multiply_60_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_32: () = density_function_Multiply_60_d5x263x5((&mut density_function_Multiply_60_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&density_function_ShiftedNoise_61_d5x263x5_output), (&minecraft_gravel_31_d5x1x5_output), (&density_function_ShiftedNoise_63_d5x263x5_output), host_consts);
        let mut density_function_Multiply_55_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_33: () = density_function_Multiply_55_d5x263x5((&mut density_function_Multiply_55_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&density_function_ShiftedNoise_56_d5x263x5_output), (&minecraft_gravel_23_d5x1x5_output), (&density_function_ShiftedNoise_58_d5x263x5_output), host_consts);
        let mut density_function_Abs_47_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_34: () = density_function_Abs_47_d5x263x5((&mut density_function_Abs_47_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_jagged_5_d5x1x5_output), host_consts);
        let mut minecraft_realism_mountains_factor_original_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_35: () = minecraft_realism_mountains_factor_original_d5x263x5((&mut minecraft_realism_mountains_factor_original_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_gravel_23_d5x1x5_output), host_consts);
        let mut minecraft_realism_mountains_slope_height_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_2_0: () = minecraft_realism_mountains_slope_height_d5x1x5((&mut minecraft_realism_mountains_slope_height_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&minecraft_cave_layer_22_d5x1x5_output), (&minecraft_gravel_23_d5x1x5_output), (&density_function_Multiply_24_d5x1x5_output), (&minecraft_realism_mountains_jagged_original_d5x1x5_output), (&minecraft_realism_mountains_factor_original_d5x1x5_output), host_consts);
        let mut minecraft_realism_extreme_mountains_slope_height_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_2_1: () = minecraft_realism_extreme_mountains_slope_height_d5x1x5((&mut minecraft_realism_extreme_mountains_slope_height_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&minecraft_cave_layer_30_d5x1x5_output), (&minecraft_gravel_31_d5x1x5_output), (&density_function_Multiply_32_d5x1x5_output), (&minecraft_realism_extreme_mountains_jagged_original_d5x1x5_output), (&minecraft_realism_extreme_mountains_factor_original_d5x1x5_output), host_consts);
        let mut minecraft_realism_extreme_mountains_slope_height_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_2_2: () = minecraft_realism_extreme_mountains_slope_height_d5x263x5((&mut minecraft_realism_extreme_mountains_slope_height_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_cave_layer_30_d5x1x5_output), (&minecraft_gravel_31_d5x1x5_output), (&density_function_Multiply_60_d5x263x5_output), (&minecraft_realism_extreme_mountains_jagged_original_d5x263x5_output), (&minecraft_realism_extreme_mountains_factor_original_d5x263x5_output), host_consts);
        let mut minecraft_caves_new_small_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_2_3: () = minecraft_caves_new_small_d5x263x5((&mut minecraft_caves_new_small_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_determiner_cave_1_d5x263x5_output), (&minecraft_cave_layer_71_d5x263x5_output), (&minecraft_jagged_72_d5x1x5_output), host_consts);
        let mut minecraft_realism_mountains_slope_height_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_2_4: () = minecraft_realism_mountains_slope_height_d5x263x5((&mut minecraft_realism_mountains_slope_height_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_cave_layer_22_d5x1x5_output), (&minecraft_gravel_23_d5x1x5_output), (&density_function_Multiply_55_d5x263x5_output), (&minecraft_realism_mountains_jagged_original_d5x263x5_output), (&minecraft_realism_mountains_factor_original_d5x263x5_output), host_consts);
        let mut density_function_Multiply_13_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_2_5: () = density_function_Multiply_13_d5x1x5((&mut density_function_Multiply_13_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&minecraft_realism_extreme_hills_factor_divergence_squared_d5x1x5_output), host_consts);
        let mut minecraft_caves_new_large_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_2_6: () = minecraft_caves_new_large_d5x263x5((&mut minecraft_caves_new_large_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_determiner_cave_1_d5x263x5_output), (&minecraft_cave_layer_74_d5x263x5_output), (&minecraft_jagged_18_d5x1x5_output), host_consts);
        let mut minecraft_caves_new_medium_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_2_7: () = minecraft_caves_new_medium_d5x263x5((&mut minecraft_caves_new_medium_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_determiner_cave_1_d5x263x5_output), (&minecraft_cave_layer_73_d5x263x5_output), (&minecraft_jagged_9_d5x1x5_output), host_consts);
        let mut density_function_Multiply_49_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_2_8: () = density_function_Multiply_49_d5x263x5((&mut density_function_Multiply_49_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_realism_extreme_hills_factor_divergence_squared_d5x263x5_output), host_consts);
        let mut density_function_Multiply_48_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_3_0: () = density_function_Multiply_48_d5x263x5((&mut density_function_Multiply_48_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_gravel_11_d5x1x5_output), (&density_function_Multiply_49_d5x263x5_output), host_consts);
        let mut density_function_Multiply_12_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_3_1: () = density_function_Multiply_12_d5x1x5((&mut density_function_Multiply_12_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&minecraft_gravel_11_d5x1x5_output), (&density_function_Multiply_13_d5x1x5_output), host_consts);
        let mut minecraft_realism_extreme_hills_slope_height_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_4_0: () = minecraft_realism_extreme_hills_slope_height_d5x263x5((&mut minecraft_realism_extreme_hills_slope_height_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_cave_layer_1_d5x1x5_output), (&minecraft_realism_extreme_hills_factor_original_d5x263x5_output), (&density_function_Multiply_48_d5x263x5_output), (&minecraft_realism_extreme_hills_jagged_original_d5x263x5_output), host_consts);
        let mut minecraft_realism_hills_slope_height_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_4_1: () = minecraft_realism_hills_slope_height_d5x1x5((&mut minecraft_realism_hills_slope_height_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&minecraft_cave_layer_10_d5x1x5_output), (&minecraft_realism_extreme_hills_factor_original_d5x1x5_output), (&density_function_Multiply_12_d5x1x5_output), (&minecraft_realism_hills_jagged_original_d5x1x5_output), (&minecraft_realism_hills_factor_original_d5x1x5_output), host_consts);
        let mut minecraft_realism_hills_slope_height_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_4_2: () = minecraft_realism_hills_slope_height_d5x263x5((&mut minecraft_realism_hills_slope_height_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_cave_layer_10_d5x1x5_output), (&minecraft_realism_extreme_hills_factor_original_d5x263x5_output), (&density_function_Multiply_48_d5x263x5_output), (&minecraft_realism_hills_jagged_original_d5x263x5_output), (&minecraft_realism_hills_factor_original_d5x263x5_output), host_consts);
        let mut minecraft_realism_extreme_hills_slope_height_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_4_3: () = minecraft_realism_extreme_hills_slope_height_d5x1x5((&mut minecraft_realism_extreme_hills_slope_height_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&minecraft_cave_layer_1_d5x1x5_output), (&minecraft_realism_extreme_hills_factor_original_d5x1x5_output), (&density_function_Multiply_12_d5x1x5_output), (&minecraft_realism_extreme_hills_jagged_original_d5x1x5_output), host_consts);
        let mut minecraft_new_surface_height_unprocessed_rivered_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_5_0: () = minecraft_new_surface_height_unprocessed_rivered_d5x1x5((&mut minecraft_new_surface_height_unprocessed_rivered_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&density_function_Add_0_d5x1x5_output), (&density_function_Abs_4_d5x1x5_output), (&density_function_Add_6_d5x1x5_output), (&minecraft_realism_hills_slope_height_d5x1x5_output), (&minecraft_realism_extreme_hills_slope_height_d5x1x5_output), (&minecraft_realism_mountains_slope_height_d5x1x5_output), (&minecraft_realism_extreme_mountains_slope_height_d5x1x5_output), (&density_function_Abs_38_d5x1x5_output), host_consts);
        let mut minecraft_new_surface_combined_unprocessed_base_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_5_1: () = minecraft_new_surface_combined_unprocessed_base_d5x263x5((&mut minecraft_new_surface_combined_unprocessed_base_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&density_function_Add_45_d5x263x5_output), (&density_function_Add_46_d5x263x5_output), (&density_function_Abs_47_d5x263x5_output), (&minecraft_realism_hills_slope_height_d5x263x5_output), (&minecraft_realism_extreme_hills_slope_height_d5x263x5_output), (&minecraft_realism_mountains_slope_height_d5x263x5_output), (&minecraft_realism_extreme_mountains_slope_height_d5x263x5_output), (&density_function_Abs_65_d5x263x5_output), host_consts);
        let mut density_function_Add_40_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_6_0: () = density_function_Add_40_d5x1x5((&mut density_function_Add_40_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&density_function_Add_0_d5x1x5_output), (&density_function_Abs_4_d5x1x5_output), (&density_function_Add_6_d5x1x5_output), (&minecraft_realism_hills_slope_height_d5x1x5_output), (&minecraft_realism_extreme_hills_slope_height_d5x1x5_output), (&minecraft_realism_mountains_slope_height_d5x1x5_output), (&minecraft_realism_extreme_mountains_slope_height_d5x1x5_output), (&density_function_Abs_38_d5x1x5_output), (&minecraft_new_surface_height_unprocessed_rivered_d5x1x5_output), host_consts);
        let mut minecraft_new_surface_combined_processed_base_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_6_1: () = minecraft_new_surface_combined_processed_base_d5x263x5((&mut minecraft_new_surface_combined_processed_base_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_new_surface_height_unprocessed_rivered_d5x1x5_output), host_consts);
        let mut minecraft_caves_underlands_combined_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_6_2: () = minecraft_caves_underlands_combined_d5x263x5((&mut minecraft_caves_underlands_combined_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_new_surface_height_unprocessed_rivered_d5x1x5_output), host_consts);
        let mut minecraft_new_surface_height_processed_base_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_6_3: () = minecraft_new_surface_height_processed_base_d5x263x5((&mut minecraft_new_surface_height_processed_base_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_new_surface_height_unprocessed_rivered_d5x1x5_output), host_consts);
        let mut minecraft_new_surface_height_unprocessed_hollowed_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_7_0: () = minecraft_new_surface_height_unprocessed_hollowed_d5x1x5((&mut minecraft_new_surface_height_unprocessed_hollowed_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&minecraft_new_surface_height_unprocessed_rivered_d5x1x5_output), (&density_function_Add_40_d5x1x5_output), host_consts);
        let mut minecraft_new_surface_combined_processed_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_8_0: () = minecraft_new_surface_combined_processed_d5x263x5((&mut minecraft_new_surface_combined_processed_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_new_surface_height_unprocessed_hollowed_d5x1x5_output), host_consts);
        let mut minecraft_new_combination_unfixed_2_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_9_0: () = minecraft_new_combination_unfixed_2_d5x263x5((&mut minecraft_new_combination_unfixed_2_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_new_surface_combined_processed_d5x263x5_output), (&minecraft_caves_underlands_combined_d5x263x5_output), (&minecraft_new_surface_combined_unprocessed_base_d5x263x5_output), host_consts);
        let mut minecraft_new_combination_unfixed_3_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_9_1: () = minecraft_new_combination_unfixed_3_d5x263x5((&mut minecraft_new_combination_unfixed_3_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_new_surface_combined_processed_d5x263x5_output), (&minecraft_caves_underlands_combined_d5x263x5_output), (&minecraft_new_surface_combined_unprocessed_base_d5x263x5_output), host_consts);
        let mut minecraft_new_combination_unfixed_new_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_10_0: () = minecraft_new_combination_unfixed_new_d5x263x5((&mut minecraft_new_combination_unfixed_new_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_new_combination_unfixed_3_d5x263x5_output), (&minecraft_new_combination_unfixed_2_d5x263x5_output), (&minecraft_new_surface_height_processed_base_d5x263x5_output), (&minecraft_caves_old_small_d5x263x5_output), (&minecraft_caves_barrier_2_d5x263x5_output), (&minecraft_caves_old_medium_d5x263x5_output), (&minecraft_caves_old_large_d5x263x5_output), (&minecraft_caves_underlands_combined_d5x263x5_output), (&minecraft_new_surface_combined_processed_base_d5x263x5_output), (&minecraft_caves_new_small_d5x263x5_output), (&minecraft_caves_new_medium_d5x263x5_output), (&minecraft_caves_new_large_d5x263x5_output), (&minecraft_caves_barrier_1_d5x263x5_output), (&minecraft_new_surface_combined_processed_d5x263x5_output), host_consts);
        let mut minecraft_new_combination_fixed_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_11_0: () = minecraft_new_combination_fixed_d5x263x5((&mut minecraft_new_combination_fixed_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_new_combination_unfixed_new_d5x263x5_output), (&density_function_YClampedGradient_76_d5x263x5_output), host_consts);
        let mut final_density_with_beardify_d16x2096x16_output: Box<[f64; 536576]> = make_buffer::<536576>();
        let _shader_call_12_0: () = final_density_with_beardify_d16x2096x16((&mut final_density_with_beardify_d16x2096x16_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(1_f64, 1_f64, 1_f64), (&minecraft_new_combination_fixed_d5x263x5_output), (&Beardifier_d16x2096x16_output), host_consts);
        let mut final_density_d16x2096x16_output: Box<[f64; 536576]> = make_buffer::<536576>();
        let _shader_call_12_1: () = final_density_d16x2096x16((&mut final_density_d16x2096x16_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(1_f64, 1_f64, 1_f64), (&minecraft_new_combination_fixed_d5x263x5_output), host_consts);
        return OrchestrationOutput { final_density: final_density_d16x2096x16_output, final_density_with_beardify: final_density_with_beardify_d16x2096x16_output };
    }
    
    pub fn orchestrate_final_density(origin: Vec3, perm_tables: &PermutationTables, host_consts: &HostConsts) -> Box<[f64; 536576]> {
        let mut minecraft_cave_layer_66_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_0_0: () = minecraft_cave_layer_66_d5x263x5((&mut minecraft_cave_layer_66_d5x263x5_output), origin, Vec3::new(8_f64, 8_f64, 8_f64), Vec3::new(32_f64, 64_f64, 32_f64), (&perm_tables.minecraft_cave_layer_0_octave__8), (&perm_tables.minecraft_cave_layer_1_octave__8), host_consts);
        let mut minecraft_cave_layer_75_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_0_1: () = minecraft_cave_layer_75_d5x263x5((&mut minecraft_cave_layer_75_d5x263x5_output), origin, Vec3::new(0.5_f64, 0.5_f64, 0.5_f64), Vec3::new(2_f64, 4_f64, 2_f64), (&perm_tables.minecraft_cave_layer_0_octave__8), (&perm_tables.minecraft_cave_layer_1_octave__8), host_consts);
        let mut density_function_ShiftedNoise_61_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_0_2: () = density_function_ShiftedNoise_61_d5x263x5((&mut density_function_ShiftedNoise_61_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&perm_tables.minecraft_gravel_0_octave__8), (&perm_tables.minecraft_gravel_1_octave__8), (&perm_tables.minecraft_gravel_0_octave__7), (&perm_tables.minecraft_gravel_1_octave__7), (&perm_tables.minecraft_gravel_0_octave__6), (&perm_tables.minecraft_gravel_1_octave__6), (&perm_tables.minecraft_gravel_0_octave__5), (&perm_tables.minecraft_gravel_1_octave__5), host_consts);
        let mut minecraft_cave_layer_10_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_3: () = minecraft_cave_layer_10_d5x1x5((&mut minecraft_cave_layer_10_d5x1x5_output), origin, Vec3::new(0.15000000596046448_f64, 0_f64, 0.15000000596046448_f64), Vec3::new(0.6000000238418579_f64, 0_f64, 0.6000000238418579_f64), (&perm_tables.minecraft_cave_layer_0_octave__8), (&perm_tables.minecraft_cave_layer_1_octave__8), host_consts);
        let mut minecraft_jagged_39_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_4: () = minecraft_jagged_39_d5x1x5((&mut minecraft_jagged_39_d5x1x5_output), origin, Vec3::new(2_f64, 0_f64, 2_f64), Vec3::new(8_f64, 0_f64, 8_f64), (&perm_tables.minecraft_jagged_0_octave__16), (&perm_tables.minecraft_jagged_1_octave__16), (&perm_tables.minecraft_jagged_0_octave__15), (&perm_tables.minecraft_jagged_1_octave__15), (&perm_tables.minecraft_jagged_0_octave__14), (&perm_tables.minecraft_jagged_1_octave__14), (&perm_tables.minecraft_jagged_0_octave__13), (&perm_tables.minecraft_jagged_1_octave__13), (&perm_tables.minecraft_jagged_0_octave__12), (&perm_tables.minecraft_jagged_1_octave__12), (&perm_tables.minecraft_jagged_0_octave__11), (&perm_tables.minecraft_jagged_1_octave__11), (&perm_tables.minecraft_jagged_0_octave__10), (&perm_tables.minecraft_jagged_1_octave__10), (&perm_tables.minecraft_jagged_0_octave__9), (&perm_tables.minecraft_jagged_1_octave__9), (&perm_tables.minecraft_jagged_0_octave__8), (&perm_tables.minecraft_jagged_1_octave__8), (&perm_tables.minecraft_jagged_0_octave__7), (&perm_tables.minecraft_jagged_1_octave__7), (&perm_tables.minecraft_jagged_0_octave__6), (&perm_tables.minecraft_jagged_1_octave__6), (&perm_tables.minecraft_jagged_0_octave__5), (&perm_tables.minecraft_jagged_1_octave__5), (&perm_tables.minecraft_jagged_0_octave__4), (&perm_tables.minecraft_jagged_1_octave__4), (&perm_tables.minecraft_jagged_0_octave__3), (&perm_tables.minecraft_jagged_1_octave__3), (&perm_tables.minecraft_jagged_0_octave__2), (&perm_tables.minecraft_jagged_1_octave__2), (&perm_tables.minecraft_jagged_0_octave__1), (&perm_tables.minecraft_jagged_1_octave__1), host_consts);
        let mut minecraft_jagged_72_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_5: () = minecraft_jagged_72_d5x1x5((&mut minecraft_jagged_72_d5x1x5_output), origin, Vec3::new(1000_f64, 0_f64, 1000_f64), Vec3::new(4000_f64, 0_f64, 4000_f64), (&perm_tables.minecraft_jagged_0_octave__16), (&perm_tables.minecraft_jagged_1_octave__16), (&perm_tables.minecraft_jagged_0_octave__15), (&perm_tables.minecraft_jagged_1_octave__15), (&perm_tables.minecraft_jagged_0_octave__14), (&perm_tables.minecraft_jagged_1_octave__14), (&perm_tables.minecraft_jagged_0_octave__13), (&perm_tables.minecraft_jagged_1_octave__13), (&perm_tables.minecraft_jagged_0_octave__12), (&perm_tables.minecraft_jagged_1_octave__12), (&perm_tables.minecraft_jagged_0_octave__11), (&perm_tables.minecraft_jagged_1_octave__11), (&perm_tables.minecraft_jagged_0_octave__10), (&perm_tables.minecraft_jagged_1_octave__10), (&perm_tables.minecraft_jagged_0_octave__9), (&perm_tables.minecraft_jagged_1_octave__9), (&perm_tables.minecraft_jagged_0_octave__8), (&perm_tables.minecraft_jagged_1_octave__8), (&perm_tables.minecraft_jagged_0_octave__7), (&perm_tables.minecraft_jagged_1_octave__7), (&perm_tables.minecraft_jagged_0_octave__6), (&perm_tables.minecraft_jagged_1_octave__6), (&perm_tables.minecraft_jagged_0_octave__5), (&perm_tables.minecraft_jagged_1_octave__5), (&perm_tables.minecraft_jagged_0_octave__4), (&perm_tables.minecraft_jagged_1_octave__4), (&perm_tables.minecraft_jagged_0_octave__3), (&perm_tables.minecraft_jagged_1_octave__3), (&perm_tables.minecraft_jagged_0_octave__2), (&perm_tables.minecraft_jagged_1_octave__2), (&perm_tables.minecraft_jagged_0_octave__1), (&perm_tables.minecraft_jagged_1_octave__1), host_consts);
        let mut density_function_ShiftedNoise_58_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_0_6: () = density_function_ShiftedNoise_58_d5x263x5((&mut density_function_ShiftedNoise_58_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&perm_tables.minecraft_gravel_0_octave__8), (&perm_tables.minecraft_gravel_1_octave__8), (&perm_tables.minecraft_gravel_0_octave__7), (&perm_tables.minecraft_gravel_1_octave__7), (&perm_tables.minecraft_gravel_0_octave__6), (&perm_tables.minecraft_gravel_1_octave__6), (&perm_tables.minecraft_gravel_0_octave__5), (&perm_tables.minecraft_gravel_1_octave__5), host_consts);
        let mut density_function_ShiftedNoise_56_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_0_7: () = density_function_ShiftedNoise_56_d5x263x5((&mut density_function_ShiftedNoise_56_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&perm_tables.minecraft_gravel_0_octave__8), (&perm_tables.minecraft_gravel_1_octave__8), (&perm_tables.minecraft_gravel_0_octave__7), (&perm_tables.minecraft_gravel_1_octave__7), (&perm_tables.minecraft_gravel_0_octave__6), (&perm_tables.minecraft_gravel_1_octave__6), (&perm_tables.minecraft_gravel_0_octave__5), (&perm_tables.minecraft_gravel_1_octave__5), host_consts);
        let mut minecraft_cave_layer_69_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_0_8: () = minecraft_cave_layer_69_d5x263x5((&mut minecraft_cave_layer_69_d5x263x5_output), origin, Vec3::new(4_f64, 4_f64, 4_f64), Vec3::new(16_f64, 32_f64, 16_f64), (&perm_tables.minecraft_cave_layer_0_octave__8), (&perm_tables.minecraft_cave_layer_1_octave__8), host_consts);
        let mut density_function_ShiftedNoise_27_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_9: () = density_function_ShiftedNoise_27_d5x1x5((&mut density_function_ShiftedNoise_27_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&perm_tables.minecraft_gravel_0_octave__8), (&perm_tables.minecraft_gravel_1_octave__8), (&perm_tables.minecraft_gravel_0_octave__7), (&perm_tables.minecraft_gravel_1_octave__7), (&perm_tables.minecraft_gravel_0_octave__6), (&perm_tables.minecraft_gravel_1_octave__6), (&perm_tables.minecraft_gravel_0_octave__5), (&perm_tables.minecraft_gravel_1_octave__5), host_consts);
        let mut minecraft_cave_layer_68_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_0_10: () = minecraft_cave_layer_68_d5x263x5((&mut minecraft_cave_layer_68_d5x263x5_output), origin, Vec3::new(6_f64, 6_f64, 6_f64), Vec3::new(24_f64, 48_f64, 24_f64), (&perm_tables.minecraft_cave_layer_0_octave__8), (&perm_tables.minecraft_cave_layer_1_octave__8), host_consts);
        let mut minecraft_gravel_23_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_11: () = minecraft_gravel_23_d5x1x5((&mut minecraft_gravel_23_d5x1x5_output), origin, Vec3::new(0.10999999940395355_f64, 0_f64, 0.10999999940395355_f64), Vec3::new(0.4399999976158142_f64, 0_f64, 0.4399999976158142_f64), (&perm_tables.minecraft_gravel_0_octave__8), (&perm_tables.minecraft_gravel_1_octave__8), (&perm_tables.minecraft_gravel_0_octave__7), (&perm_tables.minecraft_gravel_1_octave__7), (&perm_tables.minecraft_gravel_0_octave__6), (&perm_tables.minecraft_gravel_1_octave__6), (&perm_tables.minecraft_gravel_0_octave__5), (&perm_tables.minecraft_gravel_1_octave__5), host_consts);
        let mut density_function_ShiftedNoise_50_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_0_12: () = density_function_ShiftedNoise_50_d5x263x5((&mut density_function_ShiftedNoise_50_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&perm_tables.minecraft_gravel_0_octave__8), (&perm_tables.minecraft_gravel_1_octave__8), (&perm_tables.minecraft_gravel_0_octave__7), (&perm_tables.minecraft_gravel_1_octave__7), (&perm_tables.minecraft_gravel_0_octave__6), (&perm_tables.minecraft_gravel_1_octave__6), (&perm_tables.minecraft_gravel_0_octave__5), (&perm_tables.minecraft_gravel_1_octave__5), host_consts);
        let mut minecraft_jagged_18_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_13: () = minecraft_jagged_18_d5x1x5((&mut minecraft_jagged_18_d5x1x5_output), origin, Vec3::new(250_f64, 0_f64, 250_f64), Vec3::new(1000_f64, 0_f64, 1000_f64), (&perm_tables.minecraft_jagged_0_octave__16), (&perm_tables.minecraft_jagged_1_octave__16), (&perm_tables.minecraft_jagged_0_octave__15), (&perm_tables.minecraft_jagged_1_octave__15), (&perm_tables.minecraft_jagged_0_octave__14), (&perm_tables.minecraft_jagged_1_octave__14), (&perm_tables.minecraft_jagged_0_octave__13), (&perm_tables.minecraft_jagged_1_octave__13), (&perm_tables.minecraft_jagged_0_octave__12), (&perm_tables.minecraft_jagged_1_octave__12), (&perm_tables.minecraft_jagged_0_octave__11), (&perm_tables.minecraft_jagged_1_octave__11), (&perm_tables.minecraft_jagged_0_octave__10), (&perm_tables.minecraft_jagged_1_octave__10), (&perm_tables.minecraft_jagged_0_octave__9), (&perm_tables.minecraft_jagged_1_octave__9), (&perm_tables.minecraft_jagged_0_octave__8), (&perm_tables.minecraft_jagged_1_octave__8), (&perm_tables.minecraft_jagged_0_octave__7), (&perm_tables.minecraft_jagged_1_octave__7), (&perm_tables.minecraft_jagged_0_octave__6), (&perm_tables.minecraft_jagged_1_octave__6), (&perm_tables.minecraft_jagged_0_octave__5), (&perm_tables.minecraft_jagged_1_octave__5), (&perm_tables.minecraft_jagged_0_octave__4), (&perm_tables.minecraft_jagged_1_octave__4), (&perm_tables.minecraft_jagged_0_octave__3), (&perm_tables.minecraft_jagged_1_octave__3), (&perm_tables.minecraft_jagged_0_octave__2), (&perm_tables.minecraft_jagged_1_octave__2), (&perm_tables.minecraft_jagged_0_octave__1), (&perm_tables.minecraft_jagged_1_octave__1), host_consts);
        let mut minecraft_gravel_2_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_14: () = minecraft_gravel_2_d5x1x5((&mut minecraft_gravel_2_d5x1x5_output), origin, Vec3::new(0.05000000074505806_f64, 0_f64, 0.05000000074505806_f64), Vec3::new(0.20000000298023224_f64, 0_f64, 0.20000000298023224_f64), (&perm_tables.minecraft_gravel_0_octave__8), (&perm_tables.minecraft_gravel_1_octave__8), (&perm_tables.minecraft_gravel_0_octave__7), (&perm_tables.minecraft_gravel_1_octave__7), (&perm_tables.minecraft_gravel_0_octave__6), (&perm_tables.minecraft_gravel_1_octave__6), (&perm_tables.minecraft_gravel_0_octave__5), (&perm_tables.minecraft_gravel_1_octave__5), host_consts);
        let mut density_function_ShiftedNoise_63_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_0_15: () = density_function_ShiftedNoise_63_d5x263x5((&mut density_function_ShiftedNoise_63_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&perm_tables.minecraft_gravel_0_octave__8), (&perm_tables.minecraft_gravel_1_octave__8), (&perm_tables.minecraft_gravel_0_octave__7), (&perm_tables.minecraft_gravel_1_octave__7), (&perm_tables.minecraft_gravel_0_octave__6), (&perm_tables.minecraft_gravel_1_octave__6), (&perm_tables.minecraft_gravel_0_octave__5), (&perm_tables.minecraft_gravel_1_octave__5), host_consts);
        let mut minecraft_jagged_37_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_16: () = minecraft_jagged_37_d5x1x5((&mut minecraft_jagged_37_d5x1x5_output), origin, Vec3::new(600_f64, 0_f64, 600_f64), Vec3::new(2400_f64, 0_f64, 2400_f64), (&perm_tables.minecraft_jagged_0_octave__16), (&perm_tables.minecraft_jagged_1_octave__16), (&perm_tables.minecraft_jagged_0_octave__15), (&perm_tables.minecraft_jagged_1_octave__15), (&perm_tables.minecraft_jagged_0_octave__14), (&perm_tables.minecraft_jagged_1_octave__14), (&perm_tables.minecraft_jagged_0_octave__13), (&perm_tables.minecraft_jagged_1_octave__13), (&perm_tables.minecraft_jagged_0_octave__12), (&perm_tables.minecraft_jagged_1_octave__12), (&perm_tables.minecraft_jagged_0_octave__11), (&perm_tables.minecraft_jagged_1_octave__11), (&perm_tables.minecraft_jagged_0_octave__10), (&perm_tables.minecraft_jagged_1_octave__10), (&perm_tables.minecraft_jagged_0_octave__9), (&perm_tables.minecraft_jagged_1_octave__9), (&perm_tables.minecraft_jagged_0_octave__8), (&perm_tables.minecraft_jagged_1_octave__8), (&perm_tables.minecraft_jagged_0_octave__7), (&perm_tables.minecraft_jagged_1_octave__7), (&perm_tables.minecraft_jagged_0_octave__6), (&perm_tables.minecraft_jagged_1_octave__6), (&perm_tables.minecraft_jagged_0_octave__5), (&perm_tables.minecraft_jagged_1_octave__5), (&perm_tables.minecraft_jagged_0_octave__4), (&perm_tables.minecraft_jagged_1_octave__4), (&perm_tables.minecraft_jagged_0_octave__3), (&perm_tables.minecraft_jagged_1_octave__3), (&perm_tables.minecraft_jagged_0_octave__2), (&perm_tables.minecraft_jagged_1_octave__2), (&perm_tables.minecraft_jagged_0_octave__1), (&perm_tables.minecraft_jagged_1_octave__1), host_consts);
        let mut density_function_ShiftedNoise_25_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_17: () = density_function_ShiftedNoise_25_d5x1x5((&mut density_function_ShiftedNoise_25_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&perm_tables.minecraft_gravel_0_octave__8), (&perm_tables.minecraft_gravel_1_octave__8), (&perm_tables.minecraft_gravel_0_octave__7), (&perm_tables.minecraft_gravel_1_octave__7), (&perm_tables.minecraft_gravel_0_octave__6), (&perm_tables.minecraft_gravel_1_octave__6), (&perm_tables.minecraft_gravel_0_octave__5), (&perm_tables.minecraft_gravel_1_octave__5), host_consts);
        let mut minecraft_cave_layer_30_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_18: () = minecraft_cave_layer_30_d5x1x5((&mut minecraft_cave_layer_30_d5x1x5_output), origin, Vec3::new(0.25_f64, 0_f64, 0.25_f64), Vec3::new(1_f64, 0_f64, 1_f64), (&perm_tables.minecraft_cave_layer_0_octave__8), (&perm_tables.minecraft_cave_layer_1_octave__8), host_consts);
        let mut density_function_ShiftedNoise_14_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_19: () = density_function_ShiftedNoise_14_d5x1x5((&mut density_function_ShiftedNoise_14_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&perm_tables.minecraft_gravel_0_octave__8), (&perm_tables.minecraft_gravel_1_octave__8), (&perm_tables.minecraft_gravel_0_octave__7), (&perm_tables.minecraft_gravel_1_octave__7), (&perm_tables.minecraft_gravel_0_octave__6), (&perm_tables.minecraft_gravel_1_octave__6), (&perm_tables.minecraft_gravel_0_octave__5), (&perm_tables.minecraft_gravel_1_octave__5), host_consts);
        let mut minecraft_gravel_19_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_20: () = minecraft_gravel_19_d5x1x5((&mut minecraft_gravel_19_d5x1x5_output), origin, Vec3::new(0.20000000298023224_f64, 0_f64, 0.20000000298023224_f64), Vec3::new(0.800000011920929_f64, 0_f64, 0.800000011920929_f64), (&perm_tables.minecraft_gravel_0_octave__8), (&perm_tables.minecraft_gravel_1_octave__8), (&perm_tables.minecraft_gravel_0_octave__7), (&perm_tables.minecraft_gravel_1_octave__7), (&perm_tables.minecraft_gravel_0_octave__6), (&perm_tables.minecraft_gravel_1_octave__6), (&perm_tables.minecraft_gravel_0_octave__5), (&perm_tables.minecraft_gravel_1_octave__5), host_consts);
        let mut minecraft_jagged_9_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_21: () = minecraft_jagged_9_d5x1x5((&mut minecraft_jagged_9_d5x1x5_output), origin, Vec3::new(500_f64, 0_f64, 500_f64), Vec3::new(2000_f64, 0_f64, 2000_f64), (&perm_tables.minecraft_jagged_0_octave__16), (&perm_tables.minecraft_jagged_1_octave__16), (&perm_tables.minecraft_jagged_0_octave__15), (&perm_tables.minecraft_jagged_1_octave__15), (&perm_tables.minecraft_jagged_0_octave__14), (&perm_tables.minecraft_jagged_1_octave__14), (&perm_tables.minecraft_jagged_0_octave__13), (&perm_tables.minecraft_jagged_1_octave__13), (&perm_tables.minecraft_jagged_0_octave__12), (&perm_tables.minecraft_jagged_1_octave__12), (&perm_tables.minecraft_jagged_0_octave__11), (&perm_tables.minecraft_jagged_1_octave__11), (&perm_tables.minecraft_jagged_0_octave__10), (&perm_tables.minecraft_jagged_1_octave__10), (&perm_tables.minecraft_jagged_0_octave__9), (&perm_tables.minecraft_jagged_1_octave__9), (&perm_tables.minecraft_jagged_0_octave__8), (&perm_tables.minecraft_jagged_1_octave__8), (&perm_tables.minecraft_jagged_0_octave__7), (&perm_tables.minecraft_jagged_1_octave__7), (&perm_tables.minecraft_jagged_0_octave__6), (&perm_tables.minecraft_jagged_1_octave__6), (&perm_tables.minecraft_jagged_0_octave__5), (&perm_tables.minecraft_jagged_1_octave__5), (&perm_tables.minecraft_jagged_0_octave__4), (&perm_tables.minecraft_jagged_1_octave__4), (&perm_tables.minecraft_jagged_0_octave__3), (&perm_tables.minecraft_jagged_1_octave__3), (&perm_tables.minecraft_jagged_0_octave__2), (&perm_tables.minecraft_jagged_1_octave__2), (&perm_tables.minecraft_jagged_0_octave__1), (&perm_tables.minecraft_jagged_1_octave__1), host_consts);
        let mut minecraft_gravel_8_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_22: () = minecraft_gravel_8_d5x1x5((&mut minecraft_gravel_8_d5x1x5_output), origin, Vec3::new(0.10000000149011612_f64, 0_f64, 0.10000000149011612_f64), Vec3::new(0.4000000059604645_f64, 0_f64, 0.4000000059604645_f64), (&perm_tables.minecraft_gravel_0_octave__8), (&perm_tables.minecraft_gravel_1_octave__8), (&perm_tables.minecraft_gravel_0_octave__7), (&perm_tables.minecraft_gravel_1_octave__7), (&perm_tables.minecraft_gravel_0_octave__6), (&perm_tables.minecraft_gravel_1_octave__6), (&perm_tables.minecraft_gravel_0_octave__5), (&perm_tables.minecraft_gravel_1_octave__5), host_consts);
        let mut density_function_ShiftedNoise_16_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_23: () = density_function_ShiftedNoise_16_d5x1x5((&mut density_function_ShiftedNoise_16_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&perm_tables.minecraft_gravel_0_octave__8), (&perm_tables.minecraft_gravel_1_octave__8), (&perm_tables.minecraft_gravel_0_octave__7), (&perm_tables.minecraft_gravel_1_octave__7), (&perm_tables.minecraft_gravel_0_octave__6), (&perm_tables.minecraft_gravel_1_octave__6), (&perm_tables.minecraft_gravel_0_octave__5), (&perm_tables.minecraft_gravel_1_octave__5), host_consts);
        let mut minecraft_cave_layer_67_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_0_24: () = minecraft_cave_layer_67_d5x263x5((&mut minecraft_cave_layer_67_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&perm_tables.minecraft_cave_layer_0_octave__8), (&perm_tables.minecraft_cave_layer_1_octave__8), host_consts);
        let mut minecraft_jagged_5_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_25: () = minecraft_jagged_5_d5x1x5((&mut minecraft_jagged_5_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&perm_tables.minecraft_jagged_0_octave__16), (&perm_tables.minecraft_jagged_1_octave__16), (&perm_tables.minecraft_jagged_0_octave__15), (&perm_tables.minecraft_jagged_1_octave__15), (&perm_tables.minecraft_jagged_0_octave__14), (&perm_tables.minecraft_jagged_1_octave__14), (&perm_tables.minecraft_jagged_0_octave__13), (&perm_tables.minecraft_jagged_1_octave__13), (&perm_tables.minecraft_jagged_0_octave__12), (&perm_tables.minecraft_jagged_1_octave__12), (&perm_tables.minecraft_jagged_0_octave__11), (&perm_tables.minecraft_jagged_1_octave__11), (&perm_tables.minecraft_jagged_0_octave__10), (&perm_tables.minecraft_jagged_1_octave__10), (&perm_tables.minecraft_jagged_0_octave__9), (&perm_tables.minecraft_jagged_1_octave__9), (&perm_tables.minecraft_jagged_0_octave__8), (&perm_tables.minecraft_jagged_1_octave__8), (&perm_tables.minecraft_jagged_0_octave__7), (&perm_tables.minecraft_jagged_1_octave__7), (&perm_tables.minecraft_jagged_0_octave__6), (&perm_tables.minecraft_jagged_1_octave__6), (&perm_tables.minecraft_jagged_0_octave__5), (&perm_tables.minecraft_jagged_1_octave__5), (&perm_tables.minecraft_jagged_0_octave__4), (&perm_tables.minecraft_jagged_1_octave__4), (&perm_tables.minecraft_jagged_0_octave__3), (&perm_tables.minecraft_jagged_1_octave__3), (&perm_tables.minecraft_jagged_0_octave__2), (&perm_tables.minecraft_jagged_1_octave__2), (&perm_tables.minecraft_jagged_0_octave__1), (&perm_tables.minecraft_jagged_1_octave__1), host_consts);
        let mut density_function_ShiftedNoise_33_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_26: () = density_function_ShiftedNoise_33_d5x1x5((&mut density_function_ShiftedNoise_33_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&perm_tables.minecraft_gravel_0_octave__8), (&perm_tables.minecraft_gravel_1_octave__8), (&perm_tables.minecraft_gravel_0_octave__7), (&perm_tables.minecraft_gravel_1_octave__7), (&perm_tables.minecraft_gravel_0_octave__6), (&perm_tables.minecraft_gravel_1_octave__6), (&perm_tables.minecraft_gravel_0_octave__5), (&perm_tables.minecraft_gravel_1_octave__5), host_consts);
        let mut minecraft_cave_layer_22_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_27: () = minecraft_cave_layer_22_d5x1x5((&mut minecraft_cave_layer_22_d5x1x5_output), origin, Vec3::new(0.30000001192092896_f64, 0_f64, 0.30000001192092896_f64), Vec3::new(1.2000000476837158_f64, 0_f64, 1.2000000476837158_f64), (&perm_tables.minecraft_cave_layer_0_octave__8), (&perm_tables.minecraft_cave_layer_1_octave__8), host_consts);
        let mut minecraft_jagged_21_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_28: () = minecraft_jagged_21_d5x1x5((&mut minecraft_jagged_21_d5x1x5_output), origin, Vec3::new(300_f64, 0_f64, 300_f64), Vec3::new(1200_f64, 0_f64, 1200_f64), (&perm_tables.minecraft_jagged_0_octave__16), (&perm_tables.minecraft_jagged_1_octave__16), (&perm_tables.minecraft_jagged_0_octave__15), (&perm_tables.minecraft_jagged_1_octave__15), (&perm_tables.minecraft_jagged_0_octave__14), (&perm_tables.minecraft_jagged_1_octave__14), (&perm_tables.minecraft_jagged_0_octave__13), (&perm_tables.minecraft_jagged_1_octave__13), (&perm_tables.minecraft_jagged_0_octave__12), (&perm_tables.minecraft_jagged_1_octave__12), (&perm_tables.minecraft_jagged_0_octave__11), (&perm_tables.minecraft_jagged_1_octave__11), (&perm_tables.minecraft_jagged_0_octave__10), (&perm_tables.minecraft_jagged_1_octave__10), (&perm_tables.minecraft_jagged_0_octave__9), (&perm_tables.minecraft_jagged_1_octave__9), (&perm_tables.minecraft_jagged_0_octave__8), (&perm_tables.minecraft_jagged_1_octave__8), (&perm_tables.minecraft_jagged_0_octave__7), (&perm_tables.minecraft_jagged_1_octave__7), (&perm_tables.minecraft_jagged_0_octave__6), (&perm_tables.minecraft_jagged_1_octave__6), (&perm_tables.minecraft_jagged_0_octave__5), (&perm_tables.minecraft_jagged_1_octave__5), (&perm_tables.minecraft_jagged_0_octave__4), (&perm_tables.minecraft_jagged_1_octave__4), (&perm_tables.minecraft_jagged_0_octave__3), (&perm_tables.minecraft_jagged_1_octave__3), (&perm_tables.minecraft_jagged_0_octave__2), (&perm_tables.minecraft_jagged_1_octave__2), (&perm_tables.minecraft_jagged_0_octave__1), (&perm_tables.minecraft_jagged_1_octave__1), host_consts);
        let mut minecraft_gravel_11_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_29: () = minecraft_gravel_11_d5x1x5((&mut minecraft_gravel_11_d5x1x5_output), origin, Vec3::new(0.25_f64, 0_f64, 0.25_f64), Vec3::new(1_f64, 0_f64, 1_f64), (&perm_tables.minecraft_gravel_0_octave__8), (&perm_tables.minecraft_gravel_1_octave__8), (&perm_tables.minecraft_gravel_0_octave__7), (&perm_tables.minecraft_gravel_1_octave__7), (&perm_tables.minecraft_gravel_0_octave__6), (&perm_tables.minecraft_gravel_1_octave__6), (&perm_tables.minecraft_gravel_0_octave__5), (&perm_tables.minecraft_gravel_1_octave__5), host_consts);
        let mut minecraft_cave_layer_1_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_30: () = minecraft_cave_layer_1_d5x1x5((&mut minecraft_cave_layer_1_d5x1x5_output), origin, Vec3::new(0.20000000298023224_f64, 0_f64, 0.20000000298023224_f64), Vec3::new(0.800000011920929_f64, 0_f64, 0.800000011920929_f64), (&perm_tables.minecraft_cave_layer_0_octave__8), (&perm_tables.minecraft_cave_layer_1_octave__8), host_consts);
        let mut minecraft_jagged_3_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_31: () = minecraft_jagged_3_d5x1x5((&mut minecraft_jagged_3_d5x1x5_output), origin, Vec3::new(175_f64, 0_f64, 175_f64), Vec3::new(700_f64, 0_f64, 700_f64), (&perm_tables.minecraft_jagged_0_octave__16), (&perm_tables.minecraft_jagged_1_octave__16), (&perm_tables.minecraft_jagged_0_octave__15), (&perm_tables.minecraft_jagged_1_octave__15), (&perm_tables.minecraft_jagged_0_octave__14), (&perm_tables.minecraft_jagged_1_octave__14), (&perm_tables.minecraft_jagged_0_octave__13), (&perm_tables.minecraft_jagged_1_octave__13), (&perm_tables.minecraft_jagged_0_octave__12), (&perm_tables.minecraft_jagged_1_octave__12), (&perm_tables.minecraft_jagged_0_octave__11), (&perm_tables.minecraft_jagged_1_octave__11), (&perm_tables.minecraft_jagged_0_octave__10), (&perm_tables.minecraft_jagged_1_octave__10), (&perm_tables.minecraft_jagged_0_octave__9), (&perm_tables.minecraft_jagged_1_octave__9), (&perm_tables.minecraft_jagged_0_octave__8), (&perm_tables.minecraft_jagged_1_octave__8), (&perm_tables.minecraft_jagged_0_octave__7), (&perm_tables.minecraft_jagged_1_octave__7), (&perm_tables.minecraft_jagged_0_octave__6), (&perm_tables.minecraft_jagged_1_octave__6), (&perm_tables.minecraft_jagged_0_octave__5), (&perm_tables.minecraft_jagged_1_octave__5), (&perm_tables.minecraft_jagged_0_octave__4), (&perm_tables.minecraft_jagged_1_octave__4), (&perm_tables.minecraft_jagged_0_octave__3), (&perm_tables.minecraft_jagged_1_octave__3), (&perm_tables.minecraft_jagged_0_octave__2), (&perm_tables.minecraft_jagged_1_octave__2), (&perm_tables.minecraft_jagged_0_octave__1), (&perm_tables.minecraft_jagged_1_octave__1), host_consts);
        let mut minecraft_gravel_31_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_32: () = minecraft_gravel_31_d5x1x5((&mut minecraft_gravel_31_d5x1x5_output), origin, Vec3::new(0.11999999731779099_f64, 0_f64, 0.11999999731779099_f64), Vec3::new(0.47999998927116394_f64, 0_f64, 0.47999998927116394_f64), (&perm_tables.minecraft_gravel_0_octave__8), (&perm_tables.minecraft_gravel_1_octave__8), (&perm_tables.minecraft_gravel_0_octave__7), (&perm_tables.minecraft_gravel_1_octave__7), (&perm_tables.minecraft_gravel_0_octave__6), (&perm_tables.minecraft_gravel_1_octave__6), (&perm_tables.minecraft_gravel_0_octave__5), (&perm_tables.minecraft_gravel_1_octave__5), host_consts);
        let mut minecraft_cave_layer_74_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_0_33: () = minecraft_cave_layer_74_d5x263x5((&mut minecraft_cave_layer_74_d5x263x5_output), origin, Vec3::new(2.5_f64, 1_f64, 2.5_f64), Vec3::new(10_f64, 8_f64, 10_f64), (&perm_tables.minecraft_cave_layer_0_octave__8), (&perm_tables.minecraft_cave_layer_1_octave__8), host_consts);
        let mut minecraft_jagged_29_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_34: () = minecraft_jagged_29_d5x1x5((&mut minecraft_jagged_29_d5x1x5_output), origin, Vec3::new(501_f64, 0_f64, 501_f64), Vec3::new(2004_f64, 0_f64, 2004_f64), (&perm_tables.minecraft_jagged_0_octave__16), (&perm_tables.minecraft_jagged_1_octave__16), (&perm_tables.minecraft_jagged_0_octave__15), (&perm_tables.minecraft_jagged_1_octave__15), (&perm_tables.minecraft_jagged_0_octave__14), (&perm_tables.minecraft_jagged_1_octave__14), (&perm_tables.minecraft_jagged_0_octave__13), (&perm_tables.minecraft_jagged_1_octave__13), (&perm_tables.minecraft_jagged_0_octave__12), (&perm_tables.minecraft_jagged_1_octave__12), (&perm_tables.minecraft_jagged_0_octave__11), (&perm_tables.minecraft_jagged_1_octave__11), (&perm_tables.minecraft_jagged_0_octave__10), (&perm_tables.minecraft_jagged_1_octave__10), (&perm_tables.minecraft_jagged_0_octave__9), (&perm_tables.minecraft_jagged_1_octave__9), (&perm_tables.minecraft_jagged_0_octave__8), (&perm_tables.minecraft_jagged_1_octave__8), (&perm_tables.minecraft_jagged_0_octave__7), (&perm_tables.minecraft_jagged_1_octave__7), (&perm_tables.minecraft_jagged_0_octave__6), (&perm_tables.minecraft_jagged_1_octave__6), (&perm_tables.minecraft_jagged_0_octave__5), (&perm_tables.minecraft_jagged_1_octave__5), (&perm_tables.minecraft_jagged_0_octave__4), (&perm_tables.minecraft_jagged_1_octave__4), (&perm_tables.minecraft_jagged_0_octave__3), (&perm_tables.minecraft_jagged_1_octave__3), (&perm_tables.minecraft_jagged_0_octave__2), (&perm_tables.minecraft_jagged_1_octave__2), (&perm_tables.minecraft_jagged_0_octave__1), (&perm_tables.minecraft_jagged_1_octave__1), host_consts);
        let mut minecraft_cave_layer_71_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_0_35: () = minecraft_cave_layer_71_d5x263x5((&mut minecraft_cave_layer_71_d5x263x5_output), origin, Vec3::new(10_f64, 4_f64, 10_f64), Vec3::new(40_f64, 32_f64, 40_f64), (&perm_tables.minecraft_cave_layer_0_octave__8), (&perm_tables.minecraft_cave_layer_1_octave__8), host_consts);
        let mut density_function_ShiftedNoise_52_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_0_36: () = density_function_ShiftedNoise_52_d5x263x5((&mut density_function_ShiftedNoise_52_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&perm_tables.minecraft_gravel_0_octave__8), (&perm_tables.minecraft_gravel_1_octave__8), (&perm_tables.minecraft_gravel_0_octave__7), (&perm_tables.minecraft_gravel_1_octave__7), (&perm_tables.minecraft_gravel_0_octave__6), (&perm_tables.minecraft_gravel_1_octave__6), (&perm_tables.minecraft_gravel_0_octave__5), (&perm_tables.minecraft_gravel_1_octave__5), host_consts);
        let mut minecraft_cave_layer_7_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_37: () = minecraft_cave_layer_7_d5x1x5((&mut minecraft_cave_layer_7_d5x1x5_output), origin, Vec3::new(0.10000000149011612_f64, 0_f64, 0.10000000149011612_f64), Vec3::new(0.4000000059604645_f64, 0_f64, 0.4000000059604645_f64), (&perm_tables.minecraft_cave_layer_0_octave__8), (&perm_tables.minecraft_cave_layer_1_octave__8), host_consts);
        let mut density_function_ShiftedNoise_35_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_38: () = density_function_ShiftedNoise_35_d5x1x5((&mut density_function_ShiftedNoise_35_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&perm_tables.minecraft_gravel_0_octave__8), (&perm_tables.minecraft_gravel_1_octave__8), (&perm_tables.minecraft_gravel_0_octave__7), (&perm_tables.minecraft_gravel_1_octave__7), (&perm_tables.minecraft_gravel_0_octave__6), (&perm_tables.minecraft_gravel_1_octave__6), (&perm_tables.minecraft_gravel_0_octave__5), (&perm_tables.minecraft_gravel_1_octave__5), host_consts);
        let mut minecraft_gravel_70_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_39: () = minecraft_gravel_70_d5x1x5((&mut minecraft_gravel_70_d5x1x5_output), origin, Vec3::new(0.029999999329447746_f64, 0_f64, 0.029999999329447746_f64), Vec3::new(0.11999999731779099_f64, 0_f64, 0.11999999731779099_f64), (&perm_tables.minecraft_gravel_0_octave__8), (&perm_tables.minecraft_gravel_1_octave__8), (&perm_tables.minecraft_gravel_0_octave__7), (&perm_tables.minecraft_gravel_1_octave__7), (&perm_tables.minecraft_gravel_0_octave__6), (&perm_tables.minecraft_gravel_1_octave__6), (&perm_tables.minecraft_gravel_0_octave__5), (&perm_tables.minecraft_gravel_1_octave__5), host_consts);
        let mut minecraft_cave_layer_73_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_0_40: () = minecraft_cave_layer_73_d5x263x5((&mut minecraft_cave_layer_73_d5x263x5_output), origin, Vec3::new(5_f64, 2_f64, 5_f64), Vec3::new(20_f64, 16_f64, 20_f64), (&perm_tables.minecraft_cave_layer_0_octave__8), (&perm_tables.minecraft_cave_layer_1_octave__8), host_consts);
        let mut density_function_YClampedGradient_76_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_0_41: () = density_function_YClampedGradient_76_d5x263x5((&mut density_function_YClampedGradient_76_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), host_consts);
        let mut density_function_Abs_65_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_0: () = density_function_Abs_65_d5x263x5((&mut density_function_Abs_65_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_jagged_39_d5x1x5_output), host_consts);
        let mut minecraft_realism_extreme_hills_factor_original_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_1_1: () = minecraft_realism_extreme_hills_factor_original_d5x1x5((&mut minecraft_realism_extreme_hills_factor_original_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&minecraft_gravel_11_d5x1x5_output), host_consts);
        let mut minecraft_realism_extreme_mountains_jagged_original_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_1_2: () = minecraft_realism_extreme_mountains_jagged_original_d5x1x5((&mut minecraft_realism_extreme_mountains_jagged_original_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&minecraft_jagged_37_d5x1x5_output), host_consts);
        let mut minecraft_determiner_cave_1_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_3: () = minecraft_determiner_cave_1_d5x263x5((&mut minecraft_determiner_cave_1_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_gravel_70_d5x1x5_output), host_consts);
        let mut minecraft_caves_barrier_2_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_4: () = minecraft_caves_barrier_2_d5x263x5((&mut minecraft_caves_barrier_2_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_cave_layer_67_d5x263x5_output), host_consts);
        let mut minecraft_realism_extreme_hills_jagged_original_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_5: () = minecraft_realism_extreme_hills_jagged_original_d5x263x5((&mut minecraft_realism_extreme_hills_jagged_original_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_jagged_21_d5x1x5_output), host_consts);
        let mut minecraft_realism_hills_jagged_original_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_6: () = minecraft_realism_hills_jagged_original_d5x263x5((&mut minecraft_realism_hills_jagged_original_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_jagged_18_d5x1x5_output), host_consts);
        let mut minecraft_realism_extreme_hills_factor_divergence_squared_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_1_7: () = minecraft_realism_extreme_hills_factor_divergence_squared_d5x1x5((&mut minecraft_realism_extreme_hills_factor_divergence_squared_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&density_function_ShiftedNoise_14_d5x1x5_output), (&minecraft_gravel_11_d5x1x5_output), (&density_function_ShiftedNoise_16_d5x1x5_output), host_consts);
        let mut minecraft_caves_old_large_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_8: () = minecraft_caves_old_large_d5x263x5((&mut minecraft_caves_old_large_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_cave_layer_69_d5x263x5_output), host_consts);
        let mut density_function_Multiply_32_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_1_9: () = density_function_Multiply_32_d5x1x5((&mut density_function_Multiply_32_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&density_function_ShiftedNoise_33_d5x1x5_output), (&minecraft_gravel_31_d5x1x5_output), (&density_function_ShiftedNoise_35_d5x1x5_output), host_consts);
        let mut minecraft_realism_mountains_jagged_original_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_10: () = minecraft_realism_mountains_jagged_original_d5x263x5((&mut minecraft_realism_mountains_jagged_original_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_jagged_29_d5x1x5_output), host_consts);
        let mut density_function_Add_45_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_11: () = density_function_Add_45_d5x263x5((&mut density_function_Add_45_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_cave_layer_7_d5x1x5_output), (&minecraft_gravel_8_d5x1x5_output), (&minecraft_jagged_9_d5x1x5_output), host_consts);
        let mut density_function_Add_46_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_12: () = density_function_Add_46_d5x263x5((&mut density_function_Add_46_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_cave_layer_1_d5x1x5_output), (&minecraft_gravel_2_d5x1x5_output), (&minecraft_jagged_3_d5x1x5_output), host_consts);
        let mut minecraft_realism_hills_factor_original_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_1_13: () = minecraft_realism_hills_factor_original_d5x1x5((&mut minecraft_realism_hills_factor_original_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&minecraft_gravel_19_d5x1x5_output), host_consts);
        let mut density_function_Abs_38_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_1_14: () = density_function_Abs_38_d5x1x5((&mut density_function_Abs_38_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&minecraft_jagged_39_d5x1x5_output), host_consts);
        let mut minecraft_realism_mountains_jagged_original_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_1_15: () = minecraft_realism_mountains_jagged_original_d5x1x5((&mut minecraft_realism_mountains_jagged_original_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&minecraft_jagged_29_d5x1x5_output), host_consts);
        let mut density_function_Multiply_55_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_16: () = density_function_Multiply_55_d5x263x5((&mut density_function_Multiply_55_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&density_function_ShiftedNoise_56_d5x263x5_output), (&minecraft_gravel_23_d5x1x5_output), (&density_function_ShiftedNoise_58_d5x263x5_output), host_consts);
        let mut density_function_Multiply_60_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_17: () = density_function_Multiply_60_d5x263x5((&mut density_function_Multiply_60_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&density_function_ShiftedNoise_61_d5x263x5_output), (&minecraft_gravel_31_d5x1x5_output), (&density_function_ShiftedNoise_63_d5x263x5_output), host_consts);
        let mut minecraft_realism_extreme_mountains_jagged_original_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_18: () = minecraft_realism_extreme_mountains_jagged_original_d5x263x5((&mut minecraft_realism_extreme_mountains_jagged_original_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_jagged_37_d5x1x5_output), host_consts);
        let mut minecraft_realism_extreme_mountains_factor_original_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_1_19: () = minecraft_realism_extreme_mountains_factor_original_d5x1x5((&mut minecraft_realism_extreme_mountains_factor_original_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&minecraft_gravel_31_d5x1x5_output), host_consts);
        let mut density_function_Multiply_24_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_1_20: () = density_function_Multiply_24_d5x1x5((&mut density_function_Multiply_24_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&density_function_ShiftedNoise_25_d5x1x5_output), (&minecraft_gravel_23_d5x1x5_output), (&density_function_ShiftedNoise_27_d5x1x5_output), host_consts);
        let mut density_function_Add_0_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_1_21: () = density_function_Add_0_d5x1x5((&mut density_function_Add_0_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&minecraft_cave_layer_1_d5x1x5_output), (&minecraft_gravel_2_d5x1x5_output), (&minecraft_jagged_3_d5x1x5_output), host_consts);
        let mut minecraft_caves_old_small_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_22: () = minecraft_caves_old_small_d5x263x5((&mut minecraft_caves_old_small_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_cave_layer_66_d5x263x5_output), host_consts);
        let mut minecraft_realism_extreme_hills_factor_original_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_23: () = minecraft_realism_extreme_hills_factor_original_d5x263x5((&mut minecraft_realism_extreme_hills_factor_original_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_gravel_11_d5x1x5_output), host_consts);
        let mut minecraft_caves_old_medium_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_24: () = minecraft_caves_old_medium_d5x263x5((&mut minecraft_caves_old_medium_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_cave_layer_68_d5x263x5_output), host_consts);
        let mut minecraft_realism_mountains_factor_original_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_1_25: () = minecraft_realism_mountains_factor_original_d5x1x5((&mut minecraft_realism_mountains_factor_original_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&minecraft_gravel_23_d5x1x5_output), host_consts);
        let mut minecraft_realism_extreme_hills_jagged_original_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_1_26: () = minecraft_realism_extreme_hills_jagged_original_d5x1x5((&mut minecraft_realism_extreme_hills_jagged_original_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&minecraft_jagged_21_d5x1x5_output), host_consts);
        let mut density_function_Add_6_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_1_27: () = density_function_Add_6_d5x1x5((&mut density_function_Add_6_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&minecraft_cave_layer_7_d5x1x5_output), (&minecraft_gravel_8_d5x1x5_output), (&minecraft_jagged_9_d5x1x5_output), host_consts);
        let mut minecraft_realism_mountains_factor_original_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_28: () = minecraft_realism_mountains_factor_original_d5x263x5((&mut minecraft_realism_mountains_factor_original_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_gravel_23_d5x1x5_output), host_consts);
        let mut density_function_Abs_47_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_29: () = density_function_Abs_47_d5x263x5((&mut density_function_Abs_47_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_jagged_5_d5x1x5_output), host_consts);
        let mut minecraft_caves_barrier_1_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_30: () = minecraft_caves_barrier_1_d5x263x5((&mut minecraft_caves_barrier_1_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_cave_layer_75_d5x263x5_output), host_consts);
        let mut density_function_Abs_4_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_1_31: () = density_function_Abs_4_d5x1x5((&mut density_function_Abs_4_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&minecraft_jagged_5_d5x1x5_output), host_consts);
        let mut minecraft_realism_hills_jagged_original_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_1_32: () = minecraft_realism_hills_jagged_original_d5x1x5((&mut minecraft_realism_hills_jagged_original_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&minecraft_jagged_18_d5x1x5_output), host_consts);
        let mut minecraft_realism_extreme_hills_factor_divergence_squared_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_33: () = minecraft_realism_extreme_hills_factor_divergence_squared_d5x263x5((&mut minecraft_realism_extreme_hills_factor_divergence_squared_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&density_function_ShiftedNoise_50_d5x263x5_output), (&minecraft_gravel_11_d5x1x5_output), (&density_function_ShiftedNoise_52_d5x263x5_output), host_consts);
        let mut minecraft_realism_hills_factor_original_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_34: () = minecraft_realism_hills_factor_original_d5x263x5((&mut minecraft_realism_hills_factor_original_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_gravel_19_d5x1x5_output), host_consts);
        let mut minecraft_realism_extreme_mountains_factor_original_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_35: () = minecraft_realism_extreme_mountains_factor_original_d5x263x5((&mut minecraft_realism_extreme_mountains_factor_original_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_gravel_31_d5x1x5_output), host_consts);
        let mut minecraft_caves_new_medium_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_2_0: () = minecraft_caves_new_medium_d5x263x5((&mut minecraft_caves_new_medium_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_determiner_cave_1_d5x263x5_output), (&minecraft_cave_layer_73_d5x263x5_output), (&minecraft_jagged_9_d5x1x5_output), host_consts);
        let mut density_function_Multiply_49_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_2_1: () = density_function_Multiply_49_d5x263x5((&mut density_function_Multiply_49_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_realism_extreme_hills_factor_divergence_squared_d5x263x5_output), host_consts);
        let mut minecraft_realism_extreme_mountains_slope_height_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_2_2: () = minecraft_realism_extreme_mountains_slope_height_d5x263x5((&mut minecraft_realism_extreme_mountains_slope_height_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_cave_layer_30_d5x1x5_output), (&minecraft_gravel_31_d5x1x5_output), (&density_function_Multiply_60_d5x263x5_output), (&minecraft_realism_extreme_mountains_jagged_original_d5x263x5_output), (&minecraft_realism_extreme_mountains_factor_original_d5x263x5_output), host_consts);
        let mut density_function_Multiply_13_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_2_3: () = density_function_Multiply_13_d5x1x5((&mut density_function_Multiply_13_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&minecraft_realism_extreme_hills_factor_divergence_squared_d5x1x5_output), host_consts);
        let mut minecraft_realism_mountains_slope_height_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_2_4: () = minecraft_realism_mountains_slope_height_d5x263x5((&mut minecraft_realism_mountains_slope_height_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_cave_layer_22_d5x1x5_output), (&minecraft_gravel_23_d5x1x5_output), (&density_function_Multiply_55_d5x263x5_output), (&minecraft_realism_mountains_jagged_original_d5x263x5_output), (&minecraft_realism_mountains_factor_original_d5x263x5_output), host_consts);
        let mut minecraft_realism_extreme_mountains_slope_height_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_2_5: () = minecraft_realism_extreme_mountains_slope_height_d5x1x5((&mut minecraft_realism_extreme_mountains_slope_height_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&minecraft_cave_layer_30_d5x1x5_output), (&minecraft_gravel_31_d5x1x5_output), (&density_function_Multiply_32_d5x1x5_output), (&minecraft_realism_extreme_mountains_jagged_original_d5x1x5_output), (&minecraft_realism_extreme_mountains_factor_original_d5x1x5_output), host_consts);
        let mut minecraft_caves_new_large_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_2_6: () = minecraft_caves_new_large_d5x263x5((&mut minecraft_caves_new_large_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_determiner_cave_1_d5x263x5_output), (&minecraft_cave_layer_74_d5x263x5_output), (&minecraft_jagged_18_d5x1x5_output), host_consts);
        let mut minecraft_realism_mountains_slope_height_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_2_7: () = minecraft_realism_mountains_slope_height_d5x1x5((&mut minecraft_realism_mountains_slope_height_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&minecraft_cave_layer_22_d5x1x5_output), (&minecraft_gravel_23_d5x1x5_output), (&density_function_Multiply_24_d5x1x5_output), (&minecraft_realism_mountains_jagged_original_d5x1x5_output), (&minecraft_realism_mountains_factor_original_d5x1x5_output), host_consts);
        let mut minecraft_caves_new_small_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_2_8: () = minecraft_caves_new_small_d5x263x5((&mut minecraft_caves_new_small_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_determiner_cave_1_d5x263x5_output), (&minecraft_cave_layer_71_d5x263x5_output), (&minecraft_jagged_72_d5x1x5_output), host_consts);
        let mut density_function_Multiply_12_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_3_0: () = density_function_Multiply_12_d5x1x5((&mut density_function_Multiply_12_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&minecraft_gravel_11_d5x1x5_output), (&density_function_Multiply_13_d5x1x5_output), host_consts);
        let mut density_function_Multiply_48_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_3_1: () = density_function_Multiply_48_d5x263x5((&mut density_function_Multiply_48_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_gravel_11_d5x1x5_output), (&density_function_Multiply_49_d5x263x5_output), host_consts);
        let mut minecraft_realism_hills_slope_height_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_4_0: () = minecraft_realism_hills_slope_height_d5x1x5((&mut minecraft_realism_hills_slope_height_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&minecraft_cave_layer_10_d5x1x5_output), (&minecraft_realism_extreme_hills_factor_original_d5x1x5_output), (&density_function_Multiply_12_d5x1x5_output), (&minecraft_realism_hills_jagged_original_d5x1x5_output), (&minecraft_realism_hills_factor_original_d5x1x5_output), host_consts);
        let mut minecraft_realism_hills_slope_height_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_4_1: () = minecraft_realism_hills_slope_height_d5x263x5((&mut minecraft_realism_hills_slope_height_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_cave_layer_10_d5x1x5_output), (&minecraft_realism_extreme_hills_factor_original_d5x263x5_output), (&density_function_Multiply_48_d5x263x5_output), (&minecraft_realism_hills_jagged_original_d5x263x5_output), (&minecraft_realism_hills_factor_original_d5x263x5_output), host_consts);
        let mut minecraft_realism_extreme_hills_slope_height_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_4_2: () = minecraft_realism_extreme_hills_slope_height_d5x1x5((&mut minecraft_realism_extreme_hills_slope_height_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&minecraft_cave_layer_1_d5x1x5_output), (&minecraft_realism_extreme_hills_factor_original_d5x1x5_output), (&density_function_Multiply_12_d5x1x5_output), (&minecraft_realism_extreme_hills_jagged_original_d5x1x5_output), host_consts);
        let mut minecraft_realism_extreme_hills_slope_height_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_4_3: () = minecraft_realism_extreme_hills_slope_height_d5x263x5((&mut minecraft_realism_extreme_hills_slope_height_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_cave_layer_1_d5x1x5_output), (&minecraft_realism_extreme_hills_factor_original_d5x263x5_output), (&density_function_Multiply_48_d5x263x5_output), (&minecraft_realism_extreme_hills_jagged_original_d5x263x5_output), host_consts);
        let mut minecraft_new_surface_combined_unprocessed_base_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_5_0: () = minecraft_new_surface_combined_unprocessed_base_d5x263x5((&mut minecraft_new_surface_combined_unprocessed_base_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&density_function_Add_45_d5x263x5_output), (&density_function_Add_46_d5x263x5_output), (&density_function_Abs_47_d5x263x5_output), (&minecraft_realism_hills_slope_height_d5x263x5_output), (&minecraft_realism_extreme_hills_slope_height_d5x263x5_output), (&minecraft_realism_mountains_slope_height_d5x263x5_output), (&minecraft_realism_extreme_mountains_slope_height_d5x263x5_output), (&density_function_Abs_65_d5x263x5_output), host_consts);
        let mut minecraft_new_surface_height_unprocessed_rivered_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_5_1: () = minecraft_new_surface_height_unprocessed_rivered_d5x1x5((&mut minecraft_new_surface_height_unprocessed_rivered_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&density_function_Add_0_d5x1x5_output), (&density_function_Abs_4_d5x1x5_output), (&density_function_Add_6_d5x1x5_output), (&minecraft_realism_hills_slope_height_d5x1x5_output), (&minecraft_realism_extreme_hills_slope_height_d5x1x5_output), (&minecraft_realism_mountains_slope_height_d5x1x5_output), (&minecraft_realism_extreme_mountains_slope_height_d5x1x5_output), (&density_function_Abs_38_d5x1x5_output), host_consts);
        let mut minecraft_new_surface_height_processed_base_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_6_0: () = minecraft_new_surface_height_processed_base_d5x263x5((&mut minecraft_new_surface_height_processed_base_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_new_surface_height_unprocessed_rivered_d5x1x5_output), host_consts);
        let mut density_function_Add_40_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_6_1: () = density_function_Add_40_d5x1x5((&mut density_function_Add_40_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&density_function_Add_0_d5x1x5_output), (&density_function_Abs_4_d5x1x5_output), (&density_function_Add_6_d5x1x5_output), (&minecraft_realism_hills_slope_height_d5x1x5_output), (&minecraft_realism_extreme_hills_slope_height_d5x1x5_output), (&minecraft_realism_mountains_slope_height_d5x1x5_output), (&minecraft_realism_extreme_mountains_slope_height_d5x1x5_output), (&density_function_Abs_38_d5x1x5_output), (&minecraft_new_surface_height_unprocessed_rivered_d5x1x5_output), host_consts);
        let mut minecraft_new_surface_combined_processed_base_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_6_2: () = minecraft_new_surface_combined_processed_base_d5x263x5((&mut minecraft_new_surface_combined_processed_base_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_new_surface_height_unprocessed_rivered_d5x1x5_output), host_consts);
        let mut minecraft_caves_underlands_combined_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_6_3: () = minecraft_caves_underlands_combined_d5x263x5((&mut minecraft_caves_underlands_combined_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_new_surface_height_unprocessed_rivered_d5x1x5_output), host_consts);
        let mut minecraft_new_surface_height_unprocessed_hollowed_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_7_0: () = minecraft_new_surface_height_unprocessed_hollowed_d5x1x5((&mut minecraft_new_surface_height_unprocessed_hollowed_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&minecraft_new_surface_height_unprocessed_rivered_d5x1x5_output), (&density_function_Add_40_d5x1x5_output), host_consts);
        let mut minecraft_new_surface_combined_processed_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_8_0: () = minecraft_new_surface_combined_processed_d5x263x5((&mut minecraft_new_surface_combined_processed_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_new_surface_height_unprocessed_hollowed_d5x1x5_output), host_consts);
        let mut minecraft_new_combination_unfixed_2_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_9_0: () = minecraft_new_combination_unfixed_2_d5x263x5((&mut minecraft_new_combination_unfixed_2_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_new_surface_combined_processed_d5x263x5_output), (&minecraft_caves_underlands_combined_d5x263x5_output), (&minecraft_new_surface_combined_unprocessed_base_d5x263x5_output), host_consts);
        let mut minecraft_new_combination_unfixed_3_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_9_1: () = minecraft_new_combination_unfixed_3_d5x263x5((&mut minecraft_new_combination_unfixed_3_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_new_surface_combined_processed_d5x263x5_output), (&minecraft_caves_underlands_combined_d5x263x5_output), (&minecraft_new_surface_combined_unprocessed_base_d5x263x5_output), host_consts);
        let mut minecraft_new_combination_unfixed_new_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_10_0: () = minecraft_new_combination_unfixed_new_d5x263x5((&mut minecraft_new_combination_unfixed_new_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_new_combination_unfixed_3_d5x263x5_output), (&minecraft_new_combination_unfixed_2_d5x263x5_output), (&minecraft_new_surface_height_processed_base_d5x263x5_output), (&minecraft_caves_old_small_d5x263x5_output), (&minecraft_caves_barrier_2_d5x263x5_output), (&minecraft_caves_old_medium_d5x263x5_output), (&minecraft_caves_old_large_d5x263x5_output), (&minecraft_caves_underlands_combined_d5x263x5_output), (&minecraft_new_surface_combined_processed_base_d5x263x5_output), (&minecraft_caves_new_small_d5x263x5_output), (&minecraft_caves_new_medium_d5x263x5_output), (&minecraft_caves_new_large_d5x263x5_output), (&minecraft_caves_barrier_1_d5x263x5_output), (&minecraft_new_surface_combined_processed_d5x263x5_output), host_consts);
        let mut minecraft_new_combination_fixed_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_11_0: () = minecraft_new_combination_fixed_d5x263x5((&mut minecraft_new_combination_fixed_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_new_combination_unfixed_new_d5x263x5_output), (&density_function_YClampedGradient_76_d5x263x5_output), host_consts);
        let mut final_density_d16x2096x16_output: Box<[f64; 536576]> = make_buffer::<536576>();
        let _shader_call_12_0: () = final_density_d16x2096x16((&mut final_density_d16x2096x16_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(1_f64, 1_f64, 1_f64), (&minecraft_new_combination_fixed_d5x263x5_output), host_consts);
        return final_density_d16x2096x16_output;
    }
    
    pub fn orchestrate_final_density_with_beardify(origin: Vec3, perm_tables: &PermutationTables, host_consts: &HostConsts) -> Box<[f64; 536576]> {
        let mut minecraft_gravel_23_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_0: () = minecraft_gravel_23_d5x1x5((&mut minecraft_gravel_23_d5x1x5_output), origin, Vec3::new(0.10999999940395355_f64, 0_f64, 0.10999999940395355_f64), Vec3::new(0.4399999976158142_f64, 0_f64, 0.4399999976158142_f64), (&perm_tables.minecraft_gravel_0_octave__8), (&perm_tables.minecraft_gravel_1_octave__8), (&perm_tables.minecraft_gravel_0_octave__7), (&perm_tables.minecraft_gravel_1_octave__7), (&perm_tables.minecraft_gravel_0_octave__6), (&perm_tables.minecraft_gravel_1_octave__6), (&perm_tables.minecraft_gravel_0_octave__5), (&perm_tables.minecraft_gravel_1_octave__5), host_consts);
        let mut minecraft_cave_layer_68_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_0_1: () = minecraft_cave_layer_68_d5x263x5((&mut minecraft_cave_layer_68_d5x263x5_output), origin, Vec3::new(6_f64, 6_f64, 6_f64), Vec3::new(24_f64, 48_f64, 24_f64), (&perm_tables.minecraft_cave_layer_0_octave__8), (&perm_tables.minecraft_cave_layer_1_octave__8), host_consts);
        let mut density_function_ShiftedNoise_61_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_0_2: () = density_function_ShiftedNoise_61_d5x263x5((&mut density_function_ShiftedNoise_61_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&perm_tables.minecraft_gravel_0_octave__8), (&perm_tables.minecraft_gravel_1_octave__8), (&perm_tables.minecraft_gravel_0_octave__7), (&perm_tables.minecraft_gravel_1_octave__7), (&perm_tables.minecraft_gravel_0_octave__6), (&perm_tables.minecraft_gravel_1_octave__6), (&perm_tables.minecraft_gravel_0_octave__5), (&perm_tables.minecraft_gravel_1_octave__5), host_consts);
        let mut density_function_ShiftedNoise_56_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_0_3: () = density_function_ShiftedNoise_56_d5x263x5((&mut density_function_ShiftedNoise_56_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&perm_tables.minecraft_gravel_0_octave__8), (&perm_tables.minecraft_gravel_1_octave__8), (&perm_tables.minecraft_gravel_0_octave__7), (&perm_tables.minecraft_gravel_1_octave__7), (&perm_tables.minecraft_gravel_0_octave__6), (&perm_tables.minecraft_gravel_1_octave__6), (&perm_tables.minecraft_gravel_0_octave__5), (&perm_tables.minecraft_gravel_1_octave__5), host_consts);
        let mut density_function_ShiftedNoise_35_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_4: () = density_function_ShiftedNoise_35_d5x1x5((&mut density_function_ShiftedNoise_35_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&perm_tables.minecraft_gravel_0_octave__8), (&perm_tables.minecraft_gravel_1_octave__8), (&perm_tables.minecraft_gravel_0_octave__7), (&perm_tables.minecraft_gravel_1_octave__7), (&perm_tables.minecraft_gravel_0_octave__6), (&perm_tables.minecraft_gravel_1_octave__6), (&perm_tables.minecraft_gravel_0_octave__5), (&perm_tables.minecraft_gravel_1_octave__5), host_consts);
        let mut minecraft_cave_layer_73_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_0_5: () = minecraft_cave_layer_73_d5x263x5((&mut minecraft_cave_layer_73_d5x263x5_output), origin, Vec3::new(5_f64, 2_f64, 5_f64), Vec3::new(20_f64, 16_f64, 20_f64), (&perm_tables.minecraft_cave_layer_0_octave__8), (&perm_tables.minecraft_cave_layer_1_octave__8), host_consts);
        let mut minecraft_cave_layer_66_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_0_6: () = minecraft_cave_layer_66_d5x263x5((&mut minecraft_cave_layer_66_d5x263x5_output), origin, Vec3::new(8_f64, 8_f64, 8_f64), Vec3::new(32_f64, 64_f64, 32_f64), (&perm_tables.minecraft_cave_layer_0_octave__8), (&perm_tables.minecraft_cave_layer_1_octave__8), host_consts);
        let mut minecraft_jagged_72_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_7: () = minecraft_jagged_72_d5x1x5((&mut minecraft_jagged_72_d5x1x5_output), origin, Vec3::new(1000_f64, 0_f64, 1000_f64), Vec3::new(4000_f64, 0_f64, 4000_f64), (&perm_tables.minecraft_jagged_0_octave__16), (&perm_tables.minecraft_jagged_1_octave__16), (&perm_tables.minecraft_jagged_0_octave__15), (&perm_tables.minecraft_jagged_1_octave__15), (&perm_tables.minecraft_jagged_0_octave__14), (&perm_tables.minecraft_jagged_1_octave__14), (&perm_tables.minecraft_jagged_0_octave__13), (&perm_tables.minecraft_jagged_1_octave__13), (&perm_tables.minecraft_jagged_0_octave__12), (&perm_tables.minecraft_jagged_1_octave__12), (&perm_tables.minecraft_jagged_0_octave__11), (&perm_tables.minecraft_jagged_1_octave__11), (&perm_tables.minecraft_jagged_0_octave__10), (&perm_tables.minecraft_jagged_1_octave__10), (&perm_tables.minecraft_jagged_0_octave__9), (&perm_tables.minecraft_jagged_1_octave__9), (&perm_tables.minecraft_jagged_0_octave__8), (&perm_tables.minecraft_jagged_1_octave__8), (&perm_tables.minecraft_jagged_0_octave__7), (&perm_tables.minecraft_jagged_1_octave__7), (&perm_tables.minecraft_jagged_0_octave__6), (&perm_tables.minecraft_jagged_1_octave__6), (&perm_tables.minecraft_jagged_0_octave__5), (&perm_tables.minecraft_jagged_1_octave__5), (&perm_tables.minecraft_jagged_0_octave__4), (&perm_tables.minecraft_jagged_1_octave__4), (&perm_tables.minecraft_jagged_0_octave__3), (&perm_tables.minecraft_jagged_1_octave__3), (&perm_tables.minecraft_jagged_0_octave__2), (&perm_tables.minecraft_jagged_1_octave__2), (&perm_tables.minecraft_jagged_0_octave__1), (&perm_tables.minecraft_jagged_1_octave__1), host_consts);
        let mut density_function_ShiftedNoise_50_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_0_8: () = density_function_ShiftedNoise_50_d5x263x5((&mut density_function_ShiftedNoise_50_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&perm_tables.minecraft_gravel_0_octave__8), (&perm_tables.minecraft_gravel_1_octave__8), (&perm_tables.minecraft_gravel_0_octave__7), (&perm_tables.minecraft_gravel_1_octave__7), (&perm_tables.minecraft_gravel_0_octave__6), (&perm_tables.minecraft_gravel_1_octave__6), (&perm_tables.minecraft_gravel_0_octave__5), (&perm_tables.minecraft_gravel_1_octave__5), host_consts);
        let mut minecraft_cave_layer_10_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_9: () = minecraft_cave_layer_10_d5x1x5((&mut minecraft_cave_layer_10_d5x1x5_output), origin, Vec3::new(0.15000000596046448_f64, 0_f64, 0.15000000596046448_f64), Vec3::new(0.6000000238418579_f64, 0_f64, 0.6000000238418579_f64), (&perm_tables.minecraft_cave_layer_0_octave__8), (&perm_tables.minecraft_cave_layer_1_octave__8), host_consts);
        let mut density_function_ShiftedNoise_63_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_0_10: () = density_function_ShiftedNoise_63_d5x263x5((&mut density_function_ShiftedNoise_63_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&perm_tables.minecraft_gravel_0_octave__8), (&perm_tables.minecraft_gravel_1_octave__8), (&perm_tables.minecraft_gravel_0_octave__7), (&perm_tables.minecraft_gravel_1_octave__7), (&perm_tables.minecraft_gravel_0_octave__6), (&perm_tables.minecraft_gravel_1_octave__6), (&perm_tables.minecraft_gravel_0_octave__5), (&perm_tables.minecraft_gravel_1_octave__5), host_consts);
        let mut minecraft_jagged_18_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_11: () = minecraft_jagged_18_d5x1x5((&mut minecraft_jagged_18_d5x1x5_output), origin, Vec3::new(250_f64, 0_f64, 250_f64), Vec3::new(1000_f64, 0_f64, 1000_f64), (&perm_tables.minecraft_jagged_0_octave__16), (&perm_tables.minecraft_jagged_1_octave__16), (&perm_tables.minecraft_jagged_0_octave__15), (&perm_tables.minecraft_jagged_1_octave__15), (&perm_tables.minecraft_jagged_0_octave__14), (&perm_tables.minecraft_jagged_1_octave__14), (&perm_tables.minecraft_jagged_0_octave__13), (&perm_tables.minecraft_jagged_1_octave__13), (&perm_tables.minecraft_jagged_0_octave__12), (&perm_tables.minecraft_jagged_1_octave__12), (&perm_tables.minecraft_jagged_0_octave__11), (&perm_tables.minecraft_jagged_1_octave__11), (&perm_tables.minecraft_jagged_0_octave__10), (&perm_tables.minecraft_jagged_1_octave__10), (&perm_tables.minecraft_jagged_0_octave__9), (&perm_tables.minecraft_jagged_1_octave__9), (&perm_tables.minecraft_jagged_0_octave__8), (&perm_tables.minecraft_jagged_1_octave__8), (&perm_tables.minecraft_jagged_0_octave__7), (&perm_tables.minecraft_jagged_1_octave__7), (&perm_tables.minecraft_jagged_0_octave__6), (&perm_tables.minecraft_jagged_1_octave__6), (&perm_tables.minecraft_jagged_0_octave__5), (&perm_tables.minecraft_jagged_1_octave__5), (&perm_tables.minecraft_jagged_0_octave__4), (&perm_tables.minecraft_jagged_1_octave__4), (&perm_tables.minecraft_jagged_0_octave__3), (&perm_tables.minecraft_jagged_1_octave__3), (&perm_tables.minecraft_jagged_0_octave__2), (&perm_tables.minecraft_jagged_1_octave__2), (&perm_tables.minecraft_jagged_0_octave__1), (&perm_tables.minecraft_jagged_1_octave__1), host_consts);
        let mut density_function_ShiftedNoise_25_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_12: () = density_function_ShiftedNoise_25_d5x1x5((&mut density_function_ShiftedNoise_25_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&perm_tables.minecraft_gravel_0_octave__8), (&perm_tables.minecraft_gravel_1_octave__8), (&perm_tables.minecraft_gravel_0_octave__7), (&perm_tables.minecraft_gravel_1_octave__7), (&perm_tables.minecraft_gravel_0_octave__6), (&perm_tables.minecraft_gravel_1_octave__6), (&perm_tables.minecraft_gravel_0_octave__5), (&perm_tables.minecraft_gravel_1_octave__5), host_consts);
        let mut minecraft_gravel_19_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_13: () = minecraft_gravel_19_d5x1x5((&mut minecraft_gravel_19_d5x1x5_output), origin, Vec3::new(0.20000000298023224_f64, 0_f64, 0.20000000298023224_f64), Vec3::new(0.800000011920929_f64, 0_f64, 0.800000011920929_f64), (&perm_tables.minecraft_gravel_0_octave__8), (&perm_tables.minecraft_gravel_1_octave__8), (&perm_tables.minecraft_gravel_0_octave__7), (&perm_tables.minecraft_gravel_1_octave__7), (&perm_tables.minecraft_gravel_0_octave__6), (&perm_tables.minecraft_gravel_1_octave__6), (&perm_tables.minecraft_gravel_0_octave__5), (&perm_tables.minecraft_gravel_1_octave__5), host_consts);
        let mut minecraft_cave_layer_22_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_14: () = minecraft_cave_layer_22_d5x1x5((&mut minecraft_cave_layer_22_d5x1x5_output), origin, Vec3::new(0.30000001192092896_f64, 0_f64, 0.30000001192092896_f64), Vec3::new(1.2000000476837158_f64, 0_f64, 1.2000000476837158_f64), (&perm_tables.minecraft_cave_layer_0_octave__8), (&perm_tables.minecraft_cave_layer_1_octave__8), host_consts);
        let mut density_function_ShiftedNoise_14_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_15: () = density_function_ShiftedNoise_14_d5x1x5((&mut density_function_ShiftedNoise_14_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&perm_tables.minecraft_gravel_0_octave__8), (&perm_tables.minecraft_gravel_1_octave__8), (&perm_tables.minecraft_gravel_0_octave__7), (&perm_tables.minecraft_gravel_1_octave__7), (&perm_tables.minecraft_gravel_0_octave__6), (&perm_tables.minecraft_gravel_1_octave__6), (&perm_tables.minecraft_gravel_0_octave__5), (&perm_tables.minecraft_gravel_1_octave__5), host_consts);
        let mut minecraft_cave_layer_1_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_16: () = minecraft_cave_layer_1_d5x1x5((&mut minecraft_cave_layer_1_d5x1x5_output), origin, Vec3::new(0.20000000298023224_f64, 0_f64, 0.20000000298023224_f64), Vec3::new(0.800000011920929_f64, 0_f64, 0.800000011920929_f64), (&perm_tables.minecraft_cave_layer_0_octave__8), (&perm_tables.minecraft_cave_layer_1_octave__8), host_consts);
        let mut minecraft_gravel_8_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_17: () = minecraft_gravel_8_d5x1x5((&mut minecraft_gravel_8_d5x1x5_output), origin, Vec3::new(0.10000000149011612_f64, 0_f64, 0.10000000149011612_f64), Vec3::new(0.4000000059604645_f64, 0_f64, 0.4000000059604645_f64), (&perm_tables.minecraft_gravel_0_octave__8), (&perm_tables.minecraft_gravel_1_octave__8), (&perm_tables.minecraft_gravel_0_octave__7), (&perm_tables.minecraft_gravel_1_octave__7), (&perm_tables.minecraft_gravel_0_octave__6), (&perm_tables.minecraft_gravel_1_octave__6), (&perm_tables.minecraft_gravel_0_octave__5), (&perm_tables.minecraft_gravel_1_octave__5), host_consts);
        let mut minecraft_jagged_37_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_18: () = minecraft_jagged_37_d5x1x5((&mut minecraft_jagged_37_d5x1x5_output), origin, Vec3::new(600_f64, 0_f64, 600_f64), Vec3::new(2400_f64, 0_f64, 2400_f64), (&perm_tables.minecraft_jagged_0_octave__16), (&perm_tables.minecraft_jagged_1_octave__16), (&perm_tables.minecraft_jagged_0_octave__15), (&perm_tables.minecraft_jagged_1_octave__15), (&perm_tables.minecraft_jagged_0_octave__14), (&perm_tables.minecraft_jagged_1_octave__14), (&perm_tables.minecraft_jagged_0_octave__13), (&perm_tables.minecraft_jagged_1_octave__13), (&perm_tables.minecraft_jagged_0_octave__12), (&perm_tables.minecraft_jagged_1_octave__12), (&perm_tables.minecraft_jagged_0_octave__11), (&perm_tables.minecraft_jagged_1_octave__11), (&perm_tables.minecraft_jagged_0_octave__10), (&perm_tables.minecraft_jagged_1_octave__10), (&perm_tables.minecraft_jagged_0_octave__9), (&perm_tables.minecraft_jagged_1_octave__9), (&perm_tables.minecraft_jagged_0_octave__8), (&perm_tables.minecraft_jagged_1_octave__8), (&perm_tables.minecraft_jagged_0_octave__7), (&perm_tables.minecraft_jagged_1_octave__7), (&perm_tables.minecraft_jagged_0_octave__6), (&perm_tables.minecraft_jagged_1_octave__6), (&perm_tables.minecraft_jagged_0_octave__5), (&perm_tables.minecraft_jagged_1_octave__5), (&perm_tables.minecraft_jagged_0_octave__4), (&perm_tables.minecraft_jagged_1_octave__4), (&perm_tables.minecraft_jagged_0_octave__3), (&perm_tables.minecraft_jagged_1_octave__3), (&perm_tables.minecraft_jagged_0_octave__2), (&perm_tables.minecraft_jagged_1_octave__2), (&perm_tables.minecraft_jagged_0_octave__1), (&perm_tables.minecraft_jagged_1_octave__1), host_consts);
        let mut density_function_ShiftedNoise_27_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_19: () = density_function_ShiftedNoise_27_d5x1x5((&mut density_function_ShiftedNoise_27_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&perm_tables.minecraft_gravel_0_octave__8), (&perm_tables.minecraft_gravel_1_octave__8), (&perm_tables.minecraft_gravel_0_octave__7), (&perm_tables.minecraft_gravel_1_octave__7), (&perm_tables.minecraft_gravel_0_octave__6), (&perm_tables.minecraft_gravel_1_octave__6), (&perm_tables.minecraft_gravel_0_octave__5), (&perm_tables.minecraft_gravel_1_octave__5), host_consts);
        let mut minecraft_cave_layer_75_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_0_20: () = minecraft_cave_layer_75_d5x263x5((&mut minecraft_cave_layer_75_d5x263x5_output), origin, Vec3::new(0.5_f64, 0.5_f64, 0.5_f64), Vec3::new(2_f64, 4_f64, 2_f64), (&perm_tables.minecraft_cave_layer_0_octave__8), (&perm_tables.minecraft_cave_layer_1_octave__8), host_consts);
        let mut density_function_YClampedGradient_76_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_0_21: () = density_function_YClampedGradient_76_d5x263x5((&mut density_function_YClampedGradient_76_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), host_consts);
        let mut minecraft_cave_layer_7_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_22: () = minecraft_cave_layer_7_d5x1x5((&mut minecraft_cave_layer_7_d5x1x5_output), origin, Vec3::new(0.10000000149011612_f64, 0_f64, 0.10000000149011612_f64), Vec3::new(0.4000000059604645_f64, 0_f64, 0.4000000059604645_f64), (&perm_tables.minecraft_cave_layer_0_octave__8), (&perm_tables.minecraft_cave_layer_1_octave__8), host_consts);
        let mut minecraft_jagged_21_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_23: () = minecraft_jagged_21_d5x1x5((&mut minecraft_jagged_21_d5x1x5_output), origin, Vec3::new(300_f64, 0_f64, 300_f64), Vec3::new(1200_f64, 0_f64, 1200_f64), (&perm_tables.minecraft_jagged_0_octave__16), (&perm_tables.minecraft_jagged_1_octave__16), (&perm_tables.minecraft_jagged_0_octave__15), (&perm_tables.minecraft_jagged_1_octave__15), (&perm_tables.minecraft_jagged_0_octave__14), (&perm_tables.minecraft_jagged_1_octave__14), (&perm_tables.minecraft_jagged_0_octave__13), (&perm_tables.minecraft_jagged_1_octave__13), (&perm_tables.minecraft_jagged_0_octave__12), (&perm_tables.minecraft_jagged_1_octave__12), (&perm_tables.minecraft_jagged_0_octave__11), (&perm_tables.minecraft_jagged_1_octave__11), (&perm_tables.minecraft_jagged_0_octave__10), (&perm_tables.minecraft_jagged_1_octave__10), (&perm_tables.minecraft_jagged_0_octave__9), (&perm_tables.minecraft_jagged_1_octave__9), (&perm_tables.minecraft_jagged_0_octave__8), (&perm_tables.minecraft_jagged_1_octave__8), (&perm_tables.minecraft_jagged_0_octave__7), (&perm_tables.minecraft_jagged_1_octave__7), (&perm_tables.minecraft_jagged_0_octave__6), (&perm_tables.minecraft_jagged_1_octave__6), (&perm_tables.minecraft_jagged_0_octave__5), (&perm_tables.minecraft_jagged_1_octave__5), (&perm_tables.minecraft_jagged_0_octave__4), (&perm_tables.minecraft_jagged_1_octave__4), (&perm_tables.minecraft_jagged_0_octave__3), (&perm_tables.minecraft_jagged_1_octave__3), (&perm_tables.minecraft_jagged_0_octave__2), (&perm_tables.minecraft_jagged_1_octave__2), (&perm_tables.minecraft_jagged_0_octave__1), (&perm_tables.minecraft_jagged_1_octave__1), host_consts);
        let mut minecraft_gravel_2_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_24: () = minecraft_gravel_2_d5x1x5((&mut minecraft_gravel_2_d5x1x5_output), origin, Vec3::new(0.05000000074505806_f64, 0_f64, 0.05000000074505806_f64), Vec3::new(0.20000000298023224_f64, 0_f64, 0.20000000298023224_f64), (&perm_tables.minecraft_gravel_0_octave__8), (&perm_tables.minecraft_gravel_1_octave__8), (&perm_tables.minecraft_gravel_0_octave__7), (&perm_tables.minecraft_gravel_1_octave__7), (&perm_tables.minecraft_gravel_0_octave__6), (&perm_tables.minecraft_gravel_1_octave__6), (&perm_tables.minecraft_gravel_0_octave__5), (&perm_tables.minecraft_gravel_1_octave__5), host_consts);
        let mut density_function_ShiftedNoise_16_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_25: () = density_function_ShiftedNoise_16_d5x1x5((&mut density_function_ShiftedNoise_16_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&perm_tables.minecraft_gravel_0_octave__8), (&perm_tables.minecraft_gravel_1_octave__8), (&perm_tables.minecraft_gravel_0_octave__7), (&perm_tables.minecraft_gravel_1_octave__7), (&perm_tables.minecraft_gravel_0_octave__6), (&perm_tables.minecraft_gravel_1_octave__6), (&perm_tables.minecraft_gravel_0_octave__5), (&perm_tables.minecraft_gravel_1_octave__5), host_consts);
        let mut density_function_ShiftedNoise_58_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_0_26: () = density_function_ShiftedNoise_58_d5x263x5((&mut density_function_ShiftedNoise_58_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&perm_tables.minecraft_gravel_0_octave__8), (&perm_tables.minecraft_gravel_1_octave__8), (&perm_tables.minecraft_gravel_0_octave__7), (&perm_tables.minecraft_gravel_1_octave__7), (&perm_tables.minecraft_gravel_0_octave__6), (&perm_tables.minecraft_gravel_1_octave__6), (&perm_tables.minecraft_gravel_0_octave__5), (&perm_tables.minecraft_gravel_1_octave__5), host_consts);
        let mut minecraft_jagged_5_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_27: () = minecraft_jagged_5_d5x1x5((&mut minecraft_jagged_5_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&perm_tables.minecraft_jagged_0_octave__16), (&perm_tables.minecraft_jagged_1_octave__16), (&perm_tables.minecraft_jagged_0_octave__15), (&perm_tables.minecraft_jagged_1_octave__15), (&perm_tables.minecraft_jagged_0_octave__14), (&perm_tables.minecraft_jagged_1_octave__14), (&perm_tables.minecraft_jagged_0_octave__13), (&perm_tables.minecraft_jagged_1_octave__13), (&perm_tables.minecraft_jagged_0_octave__12), (&perm_tables.minecraft_jagged_1_octave__12), (&perm_tables.minecraft_jagged_0_octave__11), (&perm_tables.minecraft_jagged_1_octave__11), (&perm_tables.minecraft_jagged_0_octave__10), (&perm_tables.minecraft_jagged_1_octave__10), (&perm_tables.minecraft_jagged_0_octave__9), (&perm_tables.minecraft_jagged_1_octave__9), (&perm_tables.minecraft_jagged_0_octave__8), (&perm_tables.minecraft_jagged_1_octave__8), (&perm_tables.minecraft_jagged_0_octave__7), (&perm_tables.minecraft_jagged_1_octave__7), (&perm_tables.minecraft_jagged_0_octave__6), (&perm_tables.minecraft_jagged_1_octave__6), (&perm_tables.minecraft_jagged_0_octave__5), (&perm_tables.minecraft_jagged_1_octave__5), (&perm_tables.minecraft_jagged_0_octave__4), (&perm_tables.minecraft_jagged_1_octave__4), (&perm_tables.minecraft_jagged_0_octave__3), (&perm_tables.minecraft_jagged_1_octave__3), (&perm_tables.minecraft_jagged_0_octave__2), (&perm_tables.minecraft_jagged_1_octave__2), (&perm_tables.minecraft_jagged_0_octave__1), (&perm_tables.minecraft_jagged_1_octave__1), host_consts);
        let mut density_function_ShiftedNoise_52_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_0_28: () = density_function_ShiftedNoise_52_d5x263x5((&mut density_function_ShiftedNoise_52_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&perm_tables.minecraft_gravel_0_octave__8), (&perm_tables.minecraft_gravel_1_octave__8), (&perm_tables.minecraft_gravel_0_octave__7), (&perm_tables.minecraft_gravel_1_octave__7), (&perm_tables.minecraft_gravel_0_octave__6), (&perm_tables.minecraft_gravel_1_octave__6), (&perm_tables.minecraft_gravel_0_octave__5), (&perm_tables.minecraft_gravel_1_octave__5), host_consts);
        let mut Beardifier_d16x2096x16_output: Box<[f64; 536576]> = make_buffer::<536576>();
        let _shader_call_0_29: () = Beardifier_d16x2096x16((&mut Beardifier_d16x2096x16_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(1_f64, 1_f64, 1_f64), host_consts);
        let mut minecraft_jagged_29_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_30: () = minecraft_jagged_29_d5x1x5((&mut minecraft_jagged_29_d5x1x5_output), origin, Vec3::new(501_f64, 0_f64, 501_f64), Vec3::new(2004_f64, 0_f64, 2004_f64), (&perm_tables.minecraft_jagged_0_octave__16), (&perm_tables.minecraft_jagged_1_octave__16), (&perm_tables.minecraft_jagged_0_octave__15), (&perm_tables.minecraft_jagged_1_octave__15), (&perm_tables.minecraft_jagged_0_octave__14), (&perm_tables.minecraft_jagged_1_octave__14), (&perm_tables.minecraft_jagged_0_octave__13), (&perm_tables.minecraft_jagged_1_octave__13), (&perm_tables.minecraft_jagged_0_octave__12), (&perm_tables.minecraft_jagged_1_octave__12), (&perm_tables.minecraft_jagged_0_octave__11), (&perm_tables.minecraft_jagged_1_octave__11), (&perm_tables.minecraft_jagged_0_octave__10), (&perm_tables.minecraft_jagged_1_octave__10), (&perm_tables.minecraft_jagged_0_octave__9), (&perm_tables.minecraft_jagged_1_octave__9), (&perm_tables.minecraft_jagged_0_octave__8), (&perm_tables.minecraft_jagged_1_octave__8), (&perm_tables.minecraft_jagged_0_octave__7), (&perm_tables.minecraft_jagged_1_octave__7), (&perm_tables.minecraft_jagged_0_octave__6), (&perm_tables.minecraft_jagged_1_octave__6), (&perm_tables.minecraft_jagged_0_octave__5), (&perm_tables.minecraft_jagged_1_octave__5), (&perm_tables.minecraft_jagged_0_octave__4), (&perm_tables.minecraft_jagged_1_octave__4), (&perm_tables.minecraft_jagged_0_octave__3), (&perm_tables.minecraft_jagged_1_octave__3), (&perm_tables.minecraft_jagged_0_octave__2), (&perm_tables.minecraft_jagged_1_octave__2), (&perm_tables.minecraft_jagged_0_octave__1), (&perm_tables.minecraft_jagged_1_octave__1), host_consts);
        let mut minecraft_cave_layer_30_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_31: () = minecraft_cave_layer_30_d5x1x5((&mut minecraft_cave_layer_30_d5x1x5_output), origin, Vec3::new(0.25_f64, 0_f64, 0.25_f64), Vec3::new(1_f64, 0_f64, 1_f64), (&perm_tables.minecraft_cave_layer_0_octave__8), (&perm_tables.minecraft_cave_layer_1_octave__8), host_consts);
        let mut minecraft_gravel_31_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_32: () = minecraft_gravel_31_d5x1x5((&mut minecraft_gravel_31_d5x1x5_output), origin, Vec3::new(0.11999999731779099_f64, 0_f64, 0.11999999731779099_f64), Vec3::new(0.47999998927116394_f64, 0_f64, 0.47999998927116394_f64), (&perm_tables.minecraft_gravel_0_octave__8), (&perm_tables.minecraft_gravel_1_octave__8), (&perm_tables.minecraft_gravel_0_octave__7), (&perm_tables.minecraft_gravel_1_octave__7), (&perm_tables.minecraft_gravel_0_octave__6), (&perm_tables.minecraft_gravel_1_octave__6), (&perm_tables.minecraft_gravel_0_octave__5), (&perm_tables.minecraft_gravel_1_octave__5), host_consts);
        let mut minecraft_gravel_70_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_33: () = minecraft_gravel_70_d5x1x5((&mut minecraft_gravel_70_d5x1x5_output), origin, Vec3::new(0.029999999329447746_f64, 0_f64, 0.029999999329447746_f64), Vec3::new(0.11999999731779099_f64, 0_f64, 0.11999999731779099_f64), (&perm_tables.minecraft_gravel_0_octave__8), (&perm_tables.minecraft_gravel_1_octave__8), (&perm_tables.minecraft_gravel_0_octave__7), (&perm_tables.minecraft_gravel_1_octave__7), (&perm_tables.minecraft_gravel_0_octave__6), (&perm_tables.minecraft_gravel_1_octave__6), (&perm_tables.minecraft_gravel_0_octave__5), (&perm_tables.minecraft_gravel_1_octave__5), host_consts);
        let mut minecraft_cave_layer_69_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_0_34: () = minecraft_cave_layer_69_d5x263x5((&mut minecraft_cave_layer_69_d5x263x5_output), origin, Vec3::new(4_f64, 4_f64, 4_f64), Vec3::new(16_f64, 32_f64, 16_f64), (&perm_tables.minecraft_cave_layer_0_octave__8), (&perm_tables.minecraft_cave_layer_1_octave__8), host_consts);
        let mut minecraft_cave_layer_74_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_0_35: () = minecraft_cave_layer_74_d5x263x5((&mut minecraft_cave_layer_74_d5x263x5_output), origin, Vec3::new(2.5_f64, 1_f64, 2.5_f64), Vec3::new(10_f64, 8_f64, 10_f64), (&perm_tables.minecraft_cave_layer_0_octave__8), (&perm_tables.minecraft_cave_layer_1_octave__8), host_consts);
        let mut density_function_ShiftedNoise_33_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_36: () = density_function_ShiftedNoise_33_d5x1x5((&mut density_function_ShiftedNoise_33_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&perm_tables.minecraft_gravel_0_octave__8), (&perm_tables.minecraft_gravel_1_octave__8), (&perm_tables.minecraft_gravel_0_octave__7), (&perm_tables.minecraft_gravel_1_octave__7), (&perm_tables.minecraft_gravel_0_octave__6), (&perm_tables.minecraft_gravel_1_octave__6), (&perm_tables.minecraft_gravel_0_octave__5), (&perm_tables.minecraft_gravel_1_octave__5), host_consts);
        let mut minecraft_cave_layer_71_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_0_37: () = minecraft_cave_layer_71_d5x263x5((&mut minecraft_cave_layer_71_d5x263x5_output), origin, Vec3::new(10_f64, 4_f64, 10_f64), Vec3::new(40_f64, 32_f64, 40_f64), (&perm_tables.minecraft_cave_layer_0_octave__8), (&perm_tables.minecraft_cave_layer_1_octave__8), host_consts);
        let mut minecraft_cave_layer_67_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_0_38: () = minecraft_cave_layer_67_d5x263x5((&mut minecraft_cave_layer_67_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&perm_tables.minecraft_cave_layer_0_octave__8), (&perm_tables.minecraft_cave_layer_1_octave__8), host_consts);
        let mut minecraft_jagged_9_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_39: () = minecraft_jagged_9_d5x1x5((&mut minecraft_jagged_9_d5x1x5_output), origin, Vec3::new(500_f64, 0_f64, 500_f64), Vec3::new(2000_f64, 0_f64, 2000_f64), (&perm_tables.minecraft_jagged_0_octave__16), (&perm_tables.minecraft_jagged_1_octave__16), (&perm_tables.minecraft_jagged_0_octave__15), (&perm_tables.minecraft_jagged_1_octave__15), (&perm_tables.minecraft_jagged_0_octave__14), (&perm_tables.minecraft_jagged_1_octave__14), (&perm_tables.minecraft_jagged_0_octave__13), (&perm_tables.minecraft_jagged_1_octave__13), (&perm_tables.minecraft_jagged_0_octave__12), (&perm_tables.minecraft_jagged_1_octave__12), (&perm_tables.minecraft_jagged_0_octave__11), (&perm_tables.minecraft_jagged_1_octave__11), (&perm_tables.minecraft_jagged_0_octave__10), (&perm_tables.minecraft_jagged_1_octave__10), (&perm_tables.minecraft_jagged_0_octave__9), (&perm_tables.minecraft_jagged_1_octave__9), (&perm_tables.minecraft_jagged_0_octave__8), (&perm_tables.minecraft_jagged_1_octave__8), (&perm_tables.minecraft_jagged_0_octave__7), (&perm_tables.minecraft_jagged_1_octave__7), (&perm_tables.minecraft_jagged_0_octave__6), (&perm_tables.minecraft_jagged_1_octave__6), (&perm_tables.minecraft_jagged_0_octave__5), (&perm_tables.minecraft_jagged_1_octave__5), (&perm_tables.minecraft_jagged_0_octave__4), (&perm_tables.minecraft_jagged_1_octave__4), (&perm_tables.minecraft_jagged_0_octave__3), (&perm_tables.minecraft_jagged_1_octave__3), (&perm_tables.minecraft_jagged_0_octave__2), (&perm_tables.minecraft_jagged_1_octave__2), (&perm_tables.minecraft_jagged_0_octave__1), (&perm_tables.minecraft_jagged_1_octave__1), host_consts);
        let mut minecraft_gravel_11_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_40: () = minecraft_gravel_11_d5x1x5((&mut minecraft_gravel_11_d5x1x5_output), origin, Vec3::new(0.25_f64, 0_f64, 0.25_f64), Vec3::new(1_f64, 0_f64, 1_f64), (&perm_tables.minecraft_gravel_0_octave__8), (&perm_tables.minecraft_gravel_1_octave__8), (&perm_tables.minecraft_gravel_0_octave__7), (&perm_tables.minecraft_gravel_1_octave__7), (&perm_tables.minecraft_gravel_0_octave__6), (&perm_tables.minecraft_gravel_1_octave__6), (&perm_tables.minecraft_gravel_0_octave__5), (&perm_tables.minecraft_gravel_1_octave__5), host_consts);
        let mut minecraft_jagged_3_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_41: () = minecraft_jagged_3_d5x1x5((&mut minecraft_jagged_3_d5x1x5_output), origin, Vec3::new(175_f64, 0_f64, 175_f64), Vec3::new(700_f64, 0_f64, 700_f64), (&perm_tables.minecraft_jagged_0_octave__16), (&perm_tables.minecraft_jagged_1_octave__16), (&perm_tables.minecraft_jagged_0_octave__15), (&perm_tables.minecraft_jagged_1_octave__15), (&perm_tables.minecraft_jagged_0_octave__14), (&perm_tables.minecraft_jagged_1_octave__14), (&perm_tables.minecraft_jagged_0_octave__13), (&perm_tables.minecraft_jagged_1_octave__13), (&perm_tables.minecraft_jagged_0_octave__12), (&perm_tables.minecraft_jagged_1_octave__12), (&perm_tables.minecraft_jagged_0_octave__11), (&perm_tables.minecraft_jagged_1_octave__11), (&perm_tables.minecraft_jagged_0_octave__10), (&perm_tables.minecraft_jagged_1_octave__10), (&perm_tables.minecraft_jagged_0_octave__9), (&perm_tables.minecraft_jagged_1_octave__9), (&perm_tables.minecraft_jagged_0_octave__8), (&perm_tables.minecraft_jagged_1_octave__8), (&perm_tables.minecraft_jagged_0_octave__7), (&perm_tables.minecraft_jagged_1_octave__7), (&perm_tables.minecraft_jagged_0_octave__6), (&perm_tables.minecraft_jagged_1_octave__6), (&perm_tables.minecraft_jagged_0_octave__5), (&perm_tables.minecraft_jagged_1_octave__5), (&perm_tables.minecraft_jagged_0_octave__4), (&perm_tables.minecraft_jagged_1_octave__4), (&perm_tables.minecraft_jagged_0_octave__3), (&perm_tables.minecraft_jagged_1_octave__3), (&perm_tables.minecraft_jagged_0_octave__2), (&perm_tables.minecraft_jagged_1_octave__2), (&perm_tables.minecraft_jagged_0_octave__1), (&perm_tables.minecraft_jagged_1_octave__1), host_consts);
        let mut minecraft_jagged_39_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_0_42: () = minecraft_jagged_39_d5x1x5((&mut minecraft_jagged_39_d5x1x5_output), origin, Vec3::new(2_f64, 0_f64, 2_f64), Vec3::new(8_f64, 0_f64, 8_f64), (&perm_tables.minecraft_jagged_0_octave__16), (&perm_tables.minecraft_jagged_1_octave__16), (&perm_tables.minecraft_jagged_0_octave__15), (&perm_tables.minecraft_jagged_1_octave__15), (&perm_tables.minecraft_jagged_0_octave__14), (&perm_tables.minecraft_jagged_1_octave__14), (&perm_tables.minecraft_jagged_0_octave__13), (&perm_tables.minecraft_jagged_1_octave__13), (&perm_tables.minecraft_jagged_0_octave__12), (&perm_tables.minecraft_jagged_1_octave__12), (&perm_tables.minecraft_jagged_0_octave__11), (&perm_tables.minecraft_jagged_1_octave__11), (&perm_tables.minecraft_jagged_0_octave__10), (&perm_tables.minecraft_jagged_1_octave__10), (&perm_tables.minecraft_jagged_0_octave__9), (&perm_tables.minecraft_jagged_1_octave__9), (&perm_tables.minecraft_jagged_0_octave__8), (&perm_tables.minecraft_jagged_1_octave__8), (&perm_tables.minecraft_jagged_0_octave__7), (&perm_tables.minecraft_jagged_1_octave__7), (&perm_tables.minecraft_jagged_0_octave__6), (&perm_tables.minecraft_jagged_1_octave__6), (&perm_tables.minecraft_jagged_0_octave__5), (&perm_tables.minecraft_jagged_1_octave__5), (&perm_tables.minecraft_jagged_0_octave__4), (&perm_tables.minecraft_jagged_1_octave__4), (&perm_tables.minecraft_jagged_0_octave__3), (&perm_tables.minecraft_jagged_1_octave__3), (&perm_tables.minecraft_jagged_0_octave__2), (&perm_tables.minecraft_jagged_1_octave__2), (&perm_tables.minecraft_jagged_0_octave__1), (&perm_tables.minecraft_jagged_1_octave__1), host_consts);
        let mut density_function_Add_45_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_0: () = density_function_Add_45_d5x263x5((&mut density_function_Add_45_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_cave_layer_7_d5x1x5_output), (&minecraft_gravel_8_d5x1x5_output), (&minecraft_jagged_9_d5x1x5_output), host_consts);
        let mut minecraft_caves_old_large_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_1: () = minecraft_caves_old_large_d5x263x5((&mut minecraft_caves_old_large_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_cave_layer_69_d5x263x5_output), host_consts);
        let mut minecraft_realism_mountains_factor_original_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_1_2: () = minecraft_realism_mountains_factor_original_d5x1x5((&mut minecraft_realism_mountains_factor_original_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&minecraft_gravel_23_d5x1x5_output), host_consts);
        let mut minecraft_caves_barrier_2_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_3: () = minecraft_caves_barrier_2_d5x263x5((&mut minecraft_caves_barrier_2_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_cave_layer_67_d5x263x5_output), host_consts);
        let mut minecraft_realism_mountains_factor_original_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_4: () = minecraft_realism_mountains_factor_original_d5x263x5((&mut minecraft_realism_mountains_factor_original_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_gravel_23_d5x1x5_output), host_consts);
        let mut minecraft_realism_hills_jagged_original_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_1_5: () = minecraft_realism_hills_jagged_original_d5x1x5((&mut minecraft_realism_hills_jagged_original_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&minecraft_jagged_18_d5x1x5_output), host_consts);
        let mut minecraft_realism_extreme_hills_factor_divergence_squared_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_6: () = minecraft_realism_extreme_hills_factor_divergence_squared_d5x263x5((&mut minecraft_realism_extreme_hills_factor_divergence_squared_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&density_function_ShiftedNoise_50_d5x263x5_output), (&minecraft_gravel_11_d5x1x5_output), (&density_function_ShiftedNoise_52_d5x263x5_output), host_consts);
        let mut minecraft_realism_hills_factor_original_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_7: () = minecraft_realism_hills_factor_original_d5x263x5((&mut minecraft_realism_hills_factor_original_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_gravel_19_d5x1x5_output), host_consts);
        let mut minecraft_realism_extreme_hills_factor_original_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_1_8: () = minecraft_realism_extreme_hills_factor_original_d5x1x5((&mut minecraft_realism_extreme_hills_factor_original_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&minecraft_gravel_11_d5x1x5_output), host_consts);
        let mut density_function_Abs_65_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_9: () = density_function_Abs_65_d5x263x5((&mut density_function_Abs_65_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_jagged_39_d5x1x5_output), host_consts);
        let mut minecraft_realism_extreme_mountains_factor_original_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_10: () = minecraft_realism_extreme_mountains_factor_original_d5x263x5((&mut minecraft_realism_extreme_mountains_factor_original_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_gravel_31_d5x1x5_output), host_consts);
        let mut minecraft_realism_extreme_hills_factor_original_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_11: () = minecraft_realism_extreme_hills_factor_original_d5x263x5((&mut minecraft_realism_extreme_hills_factor_original_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_gravel_11_d5x1x5_output), host_consts);
        let mut minecraft_caves_barrier_1_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_12: () = minecraft_caves_barrier_1_d5x263x5((&mut minecraft_caves_barrier_1_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_cave_layer_75_d5x263x5_output), host_consts);
        let mut density_function_Add_46_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_13: () = density_function_Add_46_d5x263x5((&mut density_function_Add_46_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_cave_layer_1_d5x1x5_output), (&minecraft_gravel_2_d5x1x5_output), (&minecraft_jagged_3_d5x1x5_output), host_consts);
        let mut density_function_Multiply_24_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_1_14: () = density_function_Multiply_24_d5x1x5((&mut density_function_Multiply_24_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&density_function_ShiftedNoise_25_d5x1x5_output), (&minecraft_gravel_23_d5x1x5_output), (&density_function_ShiftedNoise_27_d5x1x5_output), host_consts);
        let mut minecraft_realism_extreme_hills_jagged_original_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_15: () = minecraft_realism_extreme_hills_jagged_original_d5x263x5((&mut minecraft_realism_extreme_hills_jagged_original_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_jagged_21_d5x1x5_output), host_consts);
        let mut density_function_Abs_38_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_1_16: () = density_function_Abs_38_d5x1x5((&mut density_function_Abs_38_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&minecraft_jagged_39_d5x1x5_output), host_consts);
        let mut minecraft_caves_old_medium_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_17: () = minecraft_caves_old_medium_d5x263x5((&mut minecraft_caves_old_medium_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_cave_layer_68_d5x263x5_output), host_consts);
        let mut minecraft_realism_hills_jagged_original_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_18: () = minecraft_realism_hills_jagged_original_d5x263x5((&mut minecraft_realism_hills_jagged_original_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_jagged_18_d5x1x5_output), host_consts);
        let mut minecraft_realism_extreme_mountains_jagged_original_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_1_19: () = minecraft_realism_extreme_mountains_jagged_original_d5x1x5((&mut minecraft_realism_extreme_mountains_jagged_original_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&minecraft_jagged_37_d5x1x5_output), host_consts);
        let mut minecraft_realism_extreme_hills_jagged_original_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_1_20: () = minecraft_realism_extreme_hills_jagged_original_d5x1x5((&mut minecraft_realism_extreme_hills_jagged_original_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&minecraft_jagged_21_d5x1x5_output), host_consts);
        let mut minecraft_realism_extreme_mountains_factor_original_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_1_21: () = minecraft_realism_extreme_mountains_factor_original_d5x1x5((&mut minecraft_realism_extreme_mountains_factor_original_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&minecraft_gravel_31_d5x1x5_output), host_consts);
        let mut minecraft_realism_extreme_hills_factor_divergence_squared_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_1_22: () = minecraft_realism_extreme_hills_factor_divergence_squared_d5x1x5((&mut minecraft_realism_extreme_hills_factor_divergence_squared_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&density_function_ShiftedNoise_14_d5x1x5_output), (&minecraft_gravel_11_d5x1x5_output), (&density_function_ShiftedNoise_16_d5x1x5_output), host_consts);
        let mut density_function_Multiply_32_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_1_23: () = density_function_Multiply_32_d5x1x5((&mut density_function_Multiply_32_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&density_function_ShiftedNoise_33_d5x1x5_output), (&minecraft_gravel_31_d5x1x5_output), (&density_function_ShiftedNoise_35_d5x1x5_output), host_consts);
        let mut minecraft_realism_mountains_jagged_original_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_24: () = minecraft_realism_mountains_jagged_original_d5x263x5((&mut minecraft_realism_mountains_jagged_original_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_jagged_29_d5x1x5_output), host_consts);
        let mut density_function_Add_0_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_1_25: () = density_function_Add_0_d5x1x5((&mut density_function_Add_0_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&minecraft_cave_layer_1_d5x1x5_output), (&minecraft_gravel_2_d5x1x5_output), (&minecraft_jagged_3_d5x1x5_output), host_consts);
        let mut minecraft_caves_old_small_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_26: () = minecraft_caves_old_small_d5x263x5((&mut minecraft_caves_old_small_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_cave_layer_66_d5x263x5_output), host_consts);
        let mut density_function_Multiply_60_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_27: () = density_function_Multiply_60_d5x263x5((&mut density_function_Multiply_60_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&density_function_ShiftedNoise_61_d5x263x5_output), (&minecraft_gravel_31_d5x1x5_output), (&density_function_ShiftedNoise_63_d5x263x5_output), host_consts);
        let mut density_function_Add_6_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_1_28: () = density_function_Add_6_d5x1x5((&mut density_function_Add_6_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&minecraft_cave_layer_7_d5x1x5_output), (&minecraft_gravel_8_d5x1x5_output), (&minecraft_jagged_9_d5x1x5_output), host_consts);
        let mut density_function_Multiply_55_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_29: () = density_function_Multiply_55_d5x263x5((&mut density_function_Multiply_55_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&density_function_ShiftedNoise_56_d5x263x5_output), (&minecraft_gravel_23_d5x1x5_output), (&density_function_ShiftedNoise_58_d5x263x5_output), host_consts);
        let mut minecraft_realism_extreme_mountains_jagged_original_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_30: () = minecraft_realism_extreme_mountains_jagged_original_d5x263x5((&mut minecraft_realism_extreme_mountains_jagged_original_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_jagged_37_d5x1x5_output), host_consts);
        let mut density_function_Abs_47_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_31: () = density_function_Abs_47_d5x263x5((&mut density_function_Abs_47_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_jagged_5_d5x1x5_output), host_consts);
        let mut minecraft_realism_mountains_jagged_original_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_1_32: () = minecraft_realism_mountains_jagged_original_d5x1x5((&mut minecraft_realism_mountains_jagged_original_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&minecraft_jagged_29_d5x1x5_output), host_consts);
        let mut minecraft_realism_hills_factor_original_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_1_33: () = minecraft_realism_hills_factor_original_d5x1x5((&mut minecraft_realism_hills_factor_original_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&minecraft_gravel_19_d5x1x5_output), host_consts);
        let mut minecraft_determiner_cave_1_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_1_34: () = minecraft_determiner_cave_1_d5x263x5((&mut minecraft_determiner_cave_1_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_gravel_70_d5x1x5_output), host_consts);
        let mut density_function_Abs_4_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_1_35: () = density_function_Abs_4_d5x1x5((&mut density_function_Abs_4_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&minecraft_jagged_5_d5x1x5_output), host_consts);
        let mut minecraft_realism_extreme_mountains_slope_height_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_2_0: () = minecraft_realism_extreme_mountains_slope_height_d5x1x5((&mut minecraft_realism_extreme_mountains_slope_height_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&minecraft_cave_layer_30_d5x1x5_output), (&minecraft_gravel_31_d5x1x5_output), (&density_function_Multiply_32_d5x1x5_output), (&minecraft_realism_extreme_mountains_jagged_original_d5x1x5_output), (&minecraft_realism_extreme_mountains_factor_original_d5x1x5_output), host_consts);
        let mut density_function_Multiply_49_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_2_1: () = density_function_Multiply_49_d5x263x5((&mut density_function_Multiply_49_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_realism_extreme_hills_factor_divergence_squared_d5x263x5_output), host_consts);
        let mut minecraft_realism_mountains_slope_height_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_2_2: () = minecraft_realism_mountains_slope_height_d5x1x5((&mut minecraft_realism_mountains_slope_height_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&minecraft_cave_layer_22_d5x1x5_output), (&minecraft_gravel_23_d5x1x5_output), (&density_function_Multiply_24_d5x1x5_output), (&minecraft_realism_mountains_jagged_original_d5x1x5_output), (&minecraft_realism_mountains_factor_original_d5x1x5_output), host_consts);
        let mut density_function_Multiply_13_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_2_3: () = density_function_Multiply_13_d5x1x5((&mut density_function_Multiply_13_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&minecraft_realism_extreme_hills_factor_divergence_squared_d5x1x5_output), host_consts);
        let mut minecraft_caves_new_large_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_2_4: () = minecraft_caves_new_large_d5x263x5((&mut minecraft_caves_new_large_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_determiner_cave_1_d5x263x5_output), (&minecraft_cave_layer_74_d5x263x5_output), (&minecraft_jagged_18_d5x1x5_output), host_consts);
        let mut minecraft_caves_new_medium_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_2_5: () = minecraft_caves_new_medium_d5x263x5((&mut minecraft_caves_new_medium_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_determiner_cave_1_d5x263x5_output), (&minecraft_cave_layer_73_d5x263x5_output), (&minecraft_jagged_9_d5x1x5_output), host_consts);
        let mut minecraft_caves_new_small_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_2_6: () = minecraft_caves_new_small_d5x263x5((&mut minecraft_caves_new_small_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_determiner_cave_1_d5x263x5_output), (&minecraft_cave_layer_71_d5x263x5_output), (&minecraft_jagged_72_d5x1x5_output), host_consts);
        let mut minecraft_realism_mountains_slope_height_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_2_7: () = minecraft_realism_mountains_slope_height_d5x263x5((&mut minecraft_realism_mountains_slope_height_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_cave_layer_22_d5x1x5_output), (&minecraft_gravel_23_d5x1x5_output), (&density_function_Multiply_55_d5x263x5_output), (&minecraft_realism_mountains_jagged_original_d5x263x5_output), (&minecraft_realism_mountains_factor_original_d5x263x5_output), host_consts);
        let mut minecraft_realism_extreme_mountains_slope_height_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_2_8: () = minecraft_realism_extreme_mountains_slope_height_d5x263x5((&mut minecraft_realism_extreme_mountains_slope_height_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_cave_layer_30_d5x1x5_output), (&minecraft_gravel_31_d5x1x5_output), (&density_function_Multiply_60_d5x263x5_output), (&minecraft_realism_extreme_mountains_jagged_original_d5x263x5_output), (&minecraft_realism_extreme_mountains_factor_original_d5x263x5_output), host_consts);
        let mut density_function_Multiply_48_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_3_0: () = density_function_Multiply_48_d5x263x5((&mut density_function_Multiply_48_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_gravel_11_d5x1x5_output), (&density_function_Multiply_49_d5x263x5_output), host_consts);
        let mut density_function_Multiply_12_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_3_1: () = density_function_Multiply_12_d5x1x5((&mut density_function_Multiply_12_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&minecraft_gravel_11_d5x1x5_output), (&density_function_Multiply_13_d5x1x5_output), host_consts);
        let mut minecraft_realism_hills_slope_height_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_4_0: () = minecraft_realism_hills_slope_height_d5x263x5((&mut minecraft_realism_hills_slope_height_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_cave_layer_10_d5x1x5_output), (&minecraft_realism_extreme_hills_factor_original_d5x263x5_output), (&density_function_Multiply_48_d5x263x5_output), (&minecraft_realism_hills_jagged_original_d5x263x5_output), (&minecraft_realism_hills_factor_original_d5x263x5_output), host_consts);
        let mut minecraft_realism_extreme_hills_slope_height_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_4_1: () = minecraft_realism_extreme_hills_slope_height_d5x263x5((&mut minecraft_realism_extreme_hills_slope_height_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_cave_layer_1_d5x1x5_output), (&minecraft_realism_extreme_hills_factor_original_d5x263x5_output), (&density_function_Multiply_48_d5x263x5_output), (&minecraft_realism_extreme_hills_jagged_original_d5x263x5_output), host_consts);
        let mut minecraft_realism_hills_slope_height_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_4_2: () = minecraft_realism_hills_slope_height_d5x1x5((&mut minecraft_realism_hills_slope_height_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&minecraft_cave_layer_10_d5x1x5_output), (&minecraft_realism_extreme_hills_factor_original_d5x1x5_output), (&density_function_Multiply_12_d5x1x5_output), (&minecraft_realism_hills_jagged_original_d5x1x5_output), (&minecraft_realism_hills_factor_original_d5x1x5_output), host_consts);
        let mut minecraft_realism_extreme_hills_slope_height_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_4_3: () = minecraft_realism_extreme_hills_slope_height_d5x1x5((&mut minecraft_realism_extreme_hills_slope_height_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&minecraft_cave_layer_1_d5x1x5_output), (&minecraft_realism_extreme_hills_factor_original_d5x1x5_output), (&density_function_Multiply_12_d5x1x5_output), (&minecraft_realism_extreme_hills_jagged_original_d5x1x5_output), host_consts);
        let mut minecraft_new_surface_height_unprocessed_rivered_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_5_0: () = minecraft_new_surface_height_unprocessed_rivered_d5x1x5((&mut minecraft_new_surface_height_unprocessed_rivered_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&density_function_Add_0_d5x1x5_output), (&density_function_Abs_4_d5x1x5_output), (&density_function_Add_6_d5x1x5_output), (&minecraft_realism_hills_slope_height_d5x1x5_output), (&minecraft_realism_extreme_hills_slope_height_d5x1x5_output), (&minecraft_realism_mountains_slope_height_d5x1x5_output), (&minecraft_realism_extreme_mountains_slope_height_d5x1x5_output), (&density_function_Abs_38_d5x1x5_output), host_consts);
        let mut minecraft_new_surface_combined_unprocessed_base_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_5_1: () = minecraft_new_surface_combined_unprocessed_base_d5x263x5((&mut minecraft_new_surface_combined_unprocessed_base_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&density_function_Add_45_d5x263x5_output), (&density_function_Add_46_d5x263x5_output), (&density_function_Abs_47_d5x263x5_output), (&minecraft_realism_hills_slope_height_d5x263x5_output), (&minecraft_realism_extreme_hills_slope_height_d5x263x5_output), (&minecraft_realism_mountains_slope_height_d5x263x5_output), (&minecraft_realism_extreme_mountains_slope_height_d5x263x5_output), (&density_function_Abs_65_d5x263x5_output), host_consts);
        let mut minecraft_new_surface_combined_processed_base_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_6_0: () = minecraft_new_surface_combined_processed_base_d5x263x5((&mut minecraft_new_surface_combined_processed_base_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_new_surface_height_unprocessed_rivered_d5x1x5_output), host_consts);
        let mut minecraft_new_surface_height_processed_base_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_6_1: () = minecraft_new_surface_height_processed_base_d5x263x5((&mut minecraft_new_surface_height_processed_base_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_new_surface_height_unprocessed_rivered_d5x1x5_output), host_consts);
        let mut density_function_Add_40_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_6_2: () = density_function_Add_40_d5x1x5((&mut density_function_Add_40_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&density_function_Add_0_d5x1x5_output), (&density_function_Abs_4_d5x1x5_output), (&density_function_Add_6_d5x1x5_output), (&minecraft_realism_hills_slope_height_d5x1x5_output), (&minecraft_realism_extreme_hills_slope_height_d5x1x5_output), (&minecraft_realism_mountains_slope_height_d5x1x5_output), (&minecraft_realism_extreme_mountains_slope_height_d5x1x5_output), (&density_function_Abs_38_d5x1x5_output), (&minecraft_new_surface_height_unprocessed_rivered_d5x1x5_output), host_consts);
        let mut minecraft_caves_underlands_combined_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_6_3: () = minecraft_caves_underlands_combined_d5x263x5((&mut minecraft_caves_underlands_combined_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_new_surface_height_unprocessed_rivered_d5x1x5_output), host_consts);
        let mut minecraft_new_surface_height_unprocessed_hollowed_d5x1x5_output: Box<[f64; 25]> = make_buffer::<25>();
        let _shader_call_7_0: () = minecraft_new_surface_height_unprocessed_hollowed_d5x1x5((&mut minecraft_new_surface_height_unprocessed_hollowed_d5x1x5_output), origin, Vec3::new(1_f64, 0_f64, 1_f64), Vec3::new(4_f64, 0_f64, 4_f64), (&minecraft_new_surface_height_unprocessed_rivered_d5x1x5_output), (&density_function_Add_40_d5x1x5_output), host_consts);
        let mut minecraft_new_surface_combined_processed_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_8_0: () = minecraft_new_surface_combined_processed_d5x263x5((&mut minecraft_new_surface_combined_processed_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_new_surface_height_unprocessed_hollowed_d5x1x5_output), host_consts);
        let mut minecraft_new_combination_unfixed_3_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_9_0: () = minecraft_new_combination_unfixed_3_d5x263x5((&mut minecraft_new_combination_unfixed_3_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_new_surface_combined_processed_d5x263x5_output), (&minecraft_caves_underlands_combined_d5x263x5_output), (&minecraft_new_surface_combined_unprocessed_base_d5x263x5_output), host_consts);
        let mut minecraft_new_combination_unfixed_2_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_9_1: () = minecraft_new_combination_unfixed_2_d5x263x5((&mut minecraft_new_combination_unfixed_2_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_new_surface_combined_processed_d5x263x5_output), (&minecraft_caves_underlands_combined_d5x263x5_output), (&minecraft_new_surface_combined_unprocessed_base_d5x263x5_output), host_consts);
        let mut minecraft_new_combination_unfixed_new_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_10_0: () = minecraft_new_combination_unfixed_new_d5x263x5((&mut minecraft_new_combination_unfixed_new_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_new_combination_unfixed_3_d5x263x5_output), (&minecraft_new_combination_unfixed_2_d5x263x5_output), (&minecraft_new_surface_height_processed_base_d5x263x5_output), (&minecraft_caves_old_small_d5x263x5_output), (&minecraft_caves_barrier_2_d5x263x5_output), (&minecraft_caves_old_medium_d5x263x5_output), (&minecraft_caves_old_large_d5x263x5_output), (&minecraft_caves_underlands_combined_d5x263x5_output), (&minecraft_new_surface_combined_processed_base_d5x263x5_output), (&minecraft_caves_new_small_d5x263x5_output), (&minecraft_caves_new_medium_d5x263x5_output), (&minecraft_caves_new_large_d5x263x5_output), (&minecraft_caves_barrier_1_d5x263x5_output), (&minecraft_new_surface_combined_processed_d5x263x5_output), host_consts);
        let mut minecraft_new_combination_fixed_d5x263x5_output: Box<[f64; 6575]> = make_buffer::<6575>();
        let _shader_call_11_0: () = minecraft_new_combination_fixed_d5x263x5((&mut minecraft_new_combination_fixed_d5x263x5_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(4_f64, 8_f64, 4_f64), (&minecraft_new_combination_unfixed_new_d5x263x5_output), (&density_function_YClampedGradient_76_d5x263x5_output), host_consts);
        let mut final_density_with_beardify_d16x2096x16_output: Box<[f64; 536576]> = make_buffer::<536576>();
        let _shader_call_12_0: () = final_density_with_beardify_d16x2096x16((&mut final_density_with_beardify_d16x2096x16_output), origin, Vec3::new(1_f64, 1_f64, 1_f64), Vec3::new(1_f64, 1_f64, 1_f64), (&minecraft_new_combination_fixed_d5x263x5_output), (&Beardifier_d16x2096x16_output), host_consts);
        return final_density_with_beardify_d16x2096x16_output;
    }
    
    pub fn make_permutation_tables(seed: i64) -> PermutationTables {
        return PermutationTables { minecraft_jagged_0_octave__16: make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 0_i64, -4173781914402293358_i64, -7690085512374011798_i64), minecraft_jagged_1_octave__16: make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 1_i64, -4173781914402293358_i64, -7690085512374011798_i64), minecraft_jagged_0_octave__15: make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 0_i64, 9146326347560021237_i64, -955544588430498797_i64), minecraft_jagged_1_octave__15: make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 1_i64, 9146326347560021237_i64, -955544588430498797_i64), minecraft_jagged_0_octave__14: make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 0_i64, -288186606250610412_i64, -5235201928609399341_i64), minecraft_jagged_1_octave__14: make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 1_i64, -288186606250610412_i64, -5235201928609399341_i64), minecraft_jagged_0_octave__13: make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 0_i64, -3315623468378726710_i64, -2582188670566161829_i64), minecraft_jagged_1_octave__13: make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 1_i64, -3315623468378726710_i64, -2582188670566161829_i64), minecraft_jagged_0_octave__12: make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 0_i64, -5649521212934510990_i64, 8900461776529241512_i64), minecraft_jagged_1_octave__12: make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 1_i64, -5649521212934510990_i64, 8900461776529241512_i64), minecraft_jagged_0_octave__11: make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 0_i64, 1141530288128540355_i64, 8405022147954297016_i64), minecraft_jagged_1_octave__11: make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 1_i64, 1141530288128540355_i64, 8405022147954297016_i64), minecraft_jagged_0_octave__10: make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 0_i64, 3950544105335881394_i64, 6623051330073944938_i64), minecraft_jagged_1_octave__10: make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 1_i64, 3950544105335881394_i64, 6623051330073944938_i64), minecraft_jagged_0_octave__9: make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 0_i64, 589938935082149425_i64, 5662732952352513153_i64), minecraft_jagged_1_octave__9: make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 1_i64, 589938935082149425_i64, 5662732952352513153_i64), minecraft_jagged_0_octave__8: make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 0_i64, 1078206144088113246_i64, 5239585857299060288_i64), minecraft_jagged_1_octave__8: make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 1_i64, 1078206144088113246_i64, 5239585857299060288_i64), minecraft_jagged_0_octave__7: make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 0_i64, -1075682932162398897_i64, 2700503254851170474_i64), minecraft_jagged_1_octave__7: make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 1_i64, -1075682932162398897_i64, 2700503254851170474_i64), minecraft_jagged_0_octave__6: make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 0_i64, -1937505727046359452_i64, 6887262389667105861_i64), minecraft_jagged_1_octave__6: make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 1_i64, -1937505727046359452_i64, 6887262389667105861_i64), minecraft_jagged_0_octave__5: make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 0_i64, 7888980432583755018_i64, 3328269827262531447_i64), minecraft_jagged_1_octave__5: make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 1_i64, 7888980432583755018_i64, 3328269827262531447_i64), minecraft_jagged_0_octave__4: make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 0_i64, -4787091969620723870_i64, -4579290196374760307_i64), minecraft_jagged_1_octave__4: make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 1_i64, -4787091969620723870_i64, -4579290196374760307_i64), minecraft_jagged_0_octave__3: make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 0_i64, 6040343492819601496_i64, -4840870798921671874_i64), minecraft_jagged_1_octave__3: make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 1_i64, 6040343492819601496_i64, -4840870798921671874_i64), minecraft_jagged_0_octave__2: make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 0_i64, -5430693012044290181_i64, 162122330481997252_i64), minecraft_jagged_1_octave__2: make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 1_i64, -5430693012044290181_i64, 162122330481997252_i64), minecraft_jagged_0_octave__1: make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 0_i64, -2307493697404144120_i64, -5064731985740145495_i64), minecraft_jagged_1_octave__1: make_permutation_table(seed, -503628381433452140_i64, 8209175272455791875_i64, 1_i64, -2307493697404144120_i64, -5064731985740145495_i64), minecraft_gravel_0_octave__8: make_permutation_table(seed, 1153479925840222391_i64, 7756889976470315235_i64, 0_i64, 1078206144088113246_i64, 5239585857299060288_i64), minecraft_gravel_1_octave__8: make_permutation_table(seed, 1153479925840222391_i64, 7756889976470315235_i64, 1_i64, 1078206144088113246_i64, 5239585857299060288_i64), minecraft_gravel_0_octave__7: make_permutation_table(seed, 1153479925840222391_i64, 7756889976470315235_i64, 0_i64, -1075682932162398897_i64, 2700503254851170474_i64), minecraft_gravel_1_octave__7: make_permutation_table(seed, 1153479925840222391_i64, 7756889976470315235_i64, 1_i64, -1075682932162398897_i64, 2700503254851170474_i64), minecraft_gravel_0_octave__6: make_permutation_table(seed, 1153479925840222391_i64, 7756889976470315235_i64, 0_i64, -1937505727046359452_i64, 6887262389667105861_i64), minecraft_gravel_1_octave__6: make_permutation_table(seed, 1153479925840222391_i64, 7756889976470315235_i64, 1_i64, -1937505727046359452_i64, 6887262389667105861_i64), minecraft_gravel_0_octave__5: make_permutation_table(seed, 1153479925840222391_i64, 7756889976470315235_i64, 0_i64, 7888980432583755018_i64, 3328269827262531447_i64), minecraft_gravel_1_octave__5: make_permutation_table(seed, 1153479925840222391_i64, 7756889976470315235_i64, 1_i64, 7888980432583755018_i64, 3328269827262531447_i64), minecraft_cave_layer_0_octave__8: make_permutation_table(seed, 5620043451534219907_i64, 6506268541312725510_i64, 0_i64, 1078206144088113246_i64, 5239585857299060288_i64), minecraft_cave_layer_1_octave__8: make_permutation_table(seed, 5620043451534219907_i64, 6506268541312725510_i64, 1_i64, 1078206144088113246_i64, 5239585857299060288_i64) };
    }
    
}
