use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polygon points = "3 6 9 3 15 6 21 3 21 18 15 21 9 18 3 21" ></ polygon > < line x1 = "9" y1 = "3" y2 = "18" x2 = "9" ></ line > < line y1 = "6" x1 = "15" x2 = "15" y2 = "21" ></ line > < / > } } pub const LucideMap : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("width" , "24") , ("fill" , "none")] } ;