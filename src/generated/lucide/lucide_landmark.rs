use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "22" x1 = "3" x2 = "21" y1 = "22" ></ line > < line x2 = "6" y2 = "11" x1 = "6" y1 = "18" ></ line > < line y2 = "11" y1 = "18" x2 = "10" x1 = "10" ></ line > < line x2 = "14" y1 = "18" y2 = "11" x1 = "14" ></ line > < line y1 = "18" x1 = "18" x2 = "18" y2 = "11" ></ line > < polygon points = "12 2 20 7 4 7" ></ polygon > < / > } } pub const LUCIDE_LANDMARK : Path = Path { path : icon_path , props : & [("height" , "24") , ("width" , "24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-linecap" , "round")] } ;