use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "7 8 3 12 7 16" ></ polyline > < line y2 = "12" x1 = "21" x2 = "11" y1 = "12" ></ line > < line x2 = "11" y1 = "6" y2 = "6" x1 = "21" ></ line > < line x2 = "11" y1 = "18" y2 = "18" x1 = "21" ></ line > < / > } } pub const LUCIDE_INDENT_DECREASE : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("height" , "24") , ("stroke" , "currentColor") , ("fill" , "none")] } ;