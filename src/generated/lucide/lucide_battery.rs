use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "7" height = "10" x = "2" rx = "2" width = "16" ry = "2" ></ rect > < line y1 = "11" x1 = "22" y2 = "13" x2 = "22" ></ line > < / > } } pub const LUCIDE_BATTERY : Path = Path { path : icon_path , props : & [("fill" , "none") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-width" , "2")] } ;