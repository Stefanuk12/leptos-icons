use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "14.5 17.5 3 6 3 3 6 3 17.5 14.5" ></ polyline > < line x2 = "19" y2 = "13" x1 = "13" y1 = "19" ></ line > < line x1 = "16" y1 = "16" x2 = "20" y2 = "20" ></ line > < line x2 = "21" y1 = "21" y2 = "19" x1 = "19" ></ line > < / > } } pub const LucideSword : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-width" , "2") , ("width" , "24") , ("height" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;