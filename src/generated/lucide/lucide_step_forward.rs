use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "6" x1 = "6" y1 = "4" y2 = "20" ></ line > < polygon points = "10,4 20,12 10,20" ></ polygon > < / > } } pub const LUCIDE_STEP_FORWARD : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linecap" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor")] } ;