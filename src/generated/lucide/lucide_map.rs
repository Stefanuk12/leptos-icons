use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polygon points = "3 6 9 3 15 6 21 3 21 18 15 21 9 18 3 21" ></ polygon > < line x1 = "9" x2 = "9" y1 = "3" y2 = "18" ></ line > < line y2 = "21" y1 = "6" x2 = "15" x1 = "15" ></ line > < / > } } pub const LucideMap : Path = Path { path : icon_path , props : & [("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("height" , "24") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor")] } ;