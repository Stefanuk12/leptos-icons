use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polygon points = "3 6 9 3 15 6 21 3 21 18 15 21 9 18 3 21" ></ polygon > < line x1 = "9" x2 = "9" y1 = "3" y2 = "18" ></ line > < line x2 = "15" y1 = "6" x1 = "15" y2 = "21" ></ line > < / > } } pub const LUCIDE_MAP : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-width" , "2") , ("width" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;