use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "2" x = "2" width = "20" rx = "5" height = "20" ry = "5" ></ rect > < path d = "M16 11.37A4 4 0 1 1 12.63 8 4 4 0 0 1 16 11.37z" ></ path > < line x2 = "17.51" x1 = "17.5" y1 = "6.5" y2 = "6.5" ></ line > < / > } } pub const LUCIDE_INSTAGRAM : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("height" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linecap" , "round")] } ;