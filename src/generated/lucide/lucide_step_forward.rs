use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "6" x2 = "6" y1 = "4" y2 = "20" ></ line > < polygon points = "10,4 20,12 10,20" ></ polygon > < / > } } pub const LUCIDE_STEP_FORWARD : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor")] } ;