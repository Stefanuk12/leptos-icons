use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "4" y1 = "20" x1 = "18" x2 = "18" ></ line > < polygon points = "14,20 4,12 14,4" ></ polygon > < / > } } pub const LUCIDE_STEP_BACK : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;