use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "7" x = "2" height = "10" rx = "2" ry = "2" width = "16" ></ rect > < line y2 = "13" y1 = "11" x2 = "22" x1 = "22" ></ line > < line x1 = "6" x2 = "6" y1 = "11" y2 = "13" ></ line > < / > } } pub const LucideBatteryLow : Path = Path { path : icon_path , props : & [("height" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;