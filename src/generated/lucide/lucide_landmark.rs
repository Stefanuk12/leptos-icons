use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "3" x2 = "21" y1 = "22" y2 = "22" ></ line > < line x1 = "6" y2 = "11" x2 = "6" y1 = "18" ></ line > < line y2 = "11" x1 = "10" y1 = "18" x2 = "10" ></ line > < line x1 = "14" x2 = "14" y1 = "18" y2 = "11" ></ line > < line x1 = "18" y1 = "18" y2 = "11" x2 = "18" ></ line > < polygon points = "12 2 20 7 4 7" ></ polygon > < / > } } pub const LUCIDE_LANDMARK : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor")] } ;