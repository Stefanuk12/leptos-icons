use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "18" y1 = "20" y2 = "4" x2 = "18" ></ line > < polygon points = "14,20 4,12 14,4" ></ polygon > < / > } } pub const LUCIDE_STEP_BACK : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke" , "currentColor")] } ;