use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polygon points = "5 4 15 12 5 20 5 4" ></ polygon > < line x2 = "19" y1 = "5" x1 = "19" y2 = "19" ></ line > < / > } } pub const LUCIDE_SKIP_FORWARD : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("height" , "24") , ("width" , "24")] } ;