use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polygon points = "5 4 15 12 5 20 5 4" ></ polygon > < line y2 = "19" x1 = "19" x2 = "19" y1 = "5" ></ line > < / > } } pub const LUCIDE_SKIP_FORWARD : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke" , "currentColor") , ("width" , "24")] } ;