use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "14.5 17.5 3 6 3 3 6 3 17.5 14.5" ></ polyline > < line y1 = "19" x2 = "19" x1 = "13" y2 = "13" ></ line > < line y1 = "16" y2 = "20" x1 = "16" x2 = "20" ></ line > < line y2 = "19" x1 = "19" x2 = "21" y1 = "21" ></ line > < / > } } pub const LucideSword : Path = Path { path : icon_path , props : & [("width" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("height" , "24")] } ;