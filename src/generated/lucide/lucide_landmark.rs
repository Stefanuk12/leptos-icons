use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "21" y2 = "22" y1 = "22" x1 = "3" ></ line > < line x2 = "6" y2 = "11" x1 = "6" y1 = "18" ></ line > < line x1 = "10" x2 = "10" y2 = "11" y1 = "18" ></ line > < line y1 = "18" y2 = "11" x1 = "14" x2 = "14" ></ line > < line y1 = "18" x1 = "18" x2 = "18" y2 = "11" ></ line > < polygon points = "12 2 20 7 4 7" ></ polygon > < / > } } pub const LUCIDE_LANDMARK : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("height" , "24") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-linecap" , "round")] } ;