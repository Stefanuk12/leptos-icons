use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "3" y1 = "22" x2 = "21" y2 = "22" ></ line > < line x2 = "6" x1 = "6" y2 = "11" y1 = "18" ></ line > < line x1 = "10" x2 = "10" y2 = "11" y1 = "18" ></ line > < line x1 = "14" x2 = "14" y1 = "18" y2 = "11" ></ line > < line y1 = "18" x1 = "18" x2 = "18" y2 = "11" ></ line > < polygon points = "12 2 20 7 4 7" ></ polygon > < / > } } pub const LUCIDE_LANDMARK : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24")] } ;