use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "7 8 3 12 7 16" ></ polyline > < line y1 = "12" x2 = "11" x1 = "21" y2 = "12" ></ line > < line x2 = "11" y2 = "6" x1 = "21" y1 = "6" ></ line > < line x1 = "21" x2 = "11" y2 = "18" y1 = "18" ></ line > < / > } } pub const LucideOutdent : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke" , "currentColor")] } ;