use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "10" width = "16" x = "2" y = "7" rx = "2" ry = "2" ></ rect > < line y2 = "13" x1 = "22" x2 = "22" y1 = "11" ></ line > < line x2 = "6" x1 = "6" y2 = "13" y1 = "11" ></ line > < line y2 = "13" x1 = "10" x2 = "10" y1 = "11" ></ line > < line y2 = "13" x2 = "14" x1 = "14" y1 = "11" ></ line > < / > } } pub const LUCIDE_BATTERY_FULL : Path = Path { path : icon_path , props : & [("fill" , "none") , ("width" , "24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round")] } ;