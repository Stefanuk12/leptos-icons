use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "22" y1 = "22" x1 = "3" x2 = "21" ></ line > < line x2 = "6" y2 = "11" x1 = "6" y1 = "18" ></ line > < line x1 = "10" y1 = "18" x2 = "10" y2 = "11" ></ line > < line y2 = "11" x2 = "14" y1 = "18" x1 = "14" ></ line > < line x1 = "18" y1 = "18" y2 = "11" x2 = "18" ></ line > < polygon points = "12 2 20 7 4 7" ></ polygon > < / > } } pub const LUCIDE_LANDMARK : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor")] } ;