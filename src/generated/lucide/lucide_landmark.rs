use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "3" y2 = "22" x2 = "21" y1 = "22" ></ line > < line y2 = "11" x2 = "6" y1 = "18" x1 = "6" ></ line > < line y2 = "11" x1 = "10" y1 = "18" x2 = "10" ></ line > < line y2 = "11" x1 = "14" y1 = "18" x2 = "14" ></ line > < line x2 = "18" y2 = "11" y1 = "18" x1 = "18" ></ line > < polygon points = "12 2 20 7 4 7" ></ polygon > < / > } } pub const LUCIDE_LANDMARK : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-width" , "2") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;