use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "3 8 7 12 3 16" ></ polyline > < line y2 = "12" y1 = "12" x2 = "11" x1 = "21" ></ line > < line x2 = "11" y1 = "6" x1 = "21" y2 = "6" ></ line > < line y1 = "18" x1 = "21" x2 = "11" y2 = "18" ></ line > < / > } } pub const LUCIDE_INDENT_INCREASE : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("height" , "24") , ("fill" , "none") , ("stroke-linecap" , "round")] } ;