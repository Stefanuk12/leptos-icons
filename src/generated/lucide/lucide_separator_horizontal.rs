use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "21" x1 = "3" y2 = "12" y1 = "12" ></ line > < polyline points = "8 8 12 4 16 8" ></ polyline > < polyline points = "16 16 12 20 8 16" ></ polyline > < / > } } pub const LucideSeparatorHorizontal : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("height" , "24") , ("stroke-linecap" , "round")] } ;