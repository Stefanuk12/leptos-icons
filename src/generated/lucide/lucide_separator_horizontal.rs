use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "3" y2 = "12" x2 = "21" y1 = "12" ></ line > < polyline points = "8 8 12 4 16 8" ></ polyline > < polyline points = "16 16 12 20 8 16" ></ polyline > < / > } } pub const LucideSeparatorHorizontal : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none")] } ;