use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "4 14 10 14 10 20" ></ polyline > < polyline points = "20 10 14 10 14 4" ></ polyline > < line x2 = "21" x1 = "14" y1 = "10" y2 = "3" ></ line > < line y2 = "14" x2 = "10" y1 = "21" x1 = "3" ></ line > < / > } } pub const LucideMinimize2 : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("height" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;