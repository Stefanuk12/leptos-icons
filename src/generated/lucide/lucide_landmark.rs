use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "22" y1 = "22" x1 = "3" x2 = "21" ></ line > < line y1 = "18" x2 = "6" x1 = "6" y2 = "11" ></ line > < line y1 = "18" x1 = "10" x2 = "10" y2 = "11" ></ line > < line x1 = "14" y2 = "11" y1 = "18" x2 = "14" ></ line > < line x2 = "18" x1 = "18" y1 = "18" y2 = "11" ></ line > < polygon points = "12 2 20 7 4 7" ></ polygon > < / > } } pub const LUCIDE_LANDMARK : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("height" , "24")] } ;