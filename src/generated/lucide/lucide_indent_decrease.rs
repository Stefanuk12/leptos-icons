use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "7 8 3 12 7 16" ></ polyline > < line x1 = "21" y2 = "12" x2 = "11" y1 = "12" ></ line > < line x2 = "11" y2 = "6" y1 = "6" x1 = "21" ></ line > < line x2 = "11" x1 = "21" y1 = "18" y2 = "18" ></ line > < / > } } pub const LUCIDE_INDENT_DECREASE : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("width" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round")] } ;