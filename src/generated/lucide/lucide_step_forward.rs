use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "20" x1 = "6" y1 = "4" x2 = "6" ></ line > < polygon points = "10,4 20,12 10,20" ></ polygon > < / > } } pub const LUCIDE_STEP_FORWARD : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("height" , "24") , ("width" , "24") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linejoin" , "round")] } ;