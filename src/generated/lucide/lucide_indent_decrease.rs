use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "7 8 3 12 7 16" ></ polyline > < line y1 = "12" x1 = "21" x2 = "11" y2 = "12" ></ line > < line x1 = "21" x2 = "11" y1 = "6" y2 = "6" ></ line > < line x2 = "11" y2 = "18" y1 = "18" x1 = "21" ></ line > < / > } } pub const LUCIDE_INDENT_DECREASE : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("width" , "24") , ("stroke-width" , "2") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-linejoin" , "round")] } ;