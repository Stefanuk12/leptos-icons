use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect ry = "2" x = "2" width = "16" y = "7" height = "10" rx = "2" ></ rect > < line x1 = "22" y1 = "11" y2 = "13" x2 = "22" ></ line > < line y1 = "11" y2 = "13" x2 = "6" x1 = "6" ></ line > < line x1 = "10" x2 = "10" y1 = "11" y2 = "13" ></ line > < line y2 = "13" y1 = "11" x2 = "14" x1 = "14" ></ line > < / > } } pub const LucideBatteryFull : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor")] } ;