use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "3 8 7 12 3 16" ></ polyline > < line y1 = "12" y2 = "12" x1 = "21" x2 = "11" ></ line > < line y2 = "6" x2 = "11" x1 = "21" y1 = "6" ></ line > < line x1 = "21" y1 = "18" x2 = "11" y2 = "18" ></ line > < / > } } pub const LUCIDE_INDENT_INCREASE : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("width" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none")] } ;