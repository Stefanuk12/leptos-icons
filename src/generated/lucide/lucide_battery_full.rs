use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "10" width = "16" x = "2" y = "7" ry = "2" rx = "2" ></ rect > < line y2 = "13" y1 = "11" x1 = "22" x2 = "22" ></ line > < line x1 = "6" y1 = "11" x2 = "6" y2 = "13" ></ line > < line x1 = "10" y2 = "13" y1 = "11" x2 = "10" ></ line > < line x1 = "14" y2 = "13" x2 = "14" y1 = "11" ></ line > < / > } } pub const LucideBatteryFull : Path = Path { path : icon_path , props : & [("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("height" , "24")] } ;