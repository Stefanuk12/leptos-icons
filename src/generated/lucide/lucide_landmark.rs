use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "22" x1 = "3" y1 = "22" x2 = "21" ></ line > < line x1 = "6" y1 = "18" y2 = "11" x2 = "6" ></ line > < line y1 = "18" x1 = "10" x2 = "10" y2 = "11" ></ line > < line x1 = "14" y1 = "18" x2 = "14" y2 = "11" ></ line > < line y1 = "18" x1 = "18" x2 = "18" y2 = "11" ></ line > < polygon points = "12 2 20 7 4 7" ></ polygon > < / > } } pub const LUCIDE_LANDMARK : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("width" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linecap" , "round")] } ;