use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" y = "7" height = "10" width = "16" ry = "2" x = "2" ></ rect > < line y2 = "13" x1 = "22" x2 = "22" y1 = "11" ></ line > < / > } } pub const LUCIDE_BATTERY : Path = Path { path : icon_path , props : & [("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linejoin" , "round")] } ;