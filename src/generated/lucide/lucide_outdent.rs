use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "7 8 3 12 7 16" ></ polyline > < line x1 = "21" x2 = "11" y1 = "12" y2 = "12" ></ line > < line y1 = "6" x1 = "21" x2 = "11" y2 = "6" ></ line > < line x1 = "21" x2 = "11" y1 = "18" y2 = "18" ></ line > < / > } } pub const LucideOutdent : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor")] } ;