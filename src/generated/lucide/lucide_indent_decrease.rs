use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "7 8 3 12 7 16" ></ polyline > < line y1 = "12" y2 = "12" x1 = "21" x2 = "11" ></ line > < line y2 = "6" y1 = "6" x2 = "11" x1 = "21" ></ line > < line x2 = "11" x1 = "21" y2 = "18" y1 = "18" ></ line > < / > } } pub const LUCIDE_INDENT_DECREASE : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("height" , "24")] } ;