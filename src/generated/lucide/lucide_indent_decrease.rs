use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "7 8 3 12 7 16" ></ polyline > < line y2 = "12" x1 = "21" x2 = "11" y1 = "12" ></ line > < line x1 = "21" y1 = "6" x2 = "11" y2 = "6" ></ line > < line y2 = "18" x1 = "21" y1 = "18" x2 = "11" ></ line > < / > } } pub const LUCIDE_INDENT_DECREASE : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("width" , "24") , ("height" , "24") , ("stroke" , "currentColor")] } ;