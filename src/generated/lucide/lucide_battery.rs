use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" height = "10" y = "7" x = "2" width = "16" ry = "2" ></ rect > < line x2 = "22" y2 = "13" x1 = "22" y1 = "11" ></ line > < / > } } pub const LUCIDE_BATTERY : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-width" , "2") , ("height" , "24") , ("width" , "24") , ("viewBox" , "0 0 24 24")] } ;