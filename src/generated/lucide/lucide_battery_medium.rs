use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect ry = "2" height = "10" x = "2" rx = "2" width = "16" y = "7" ></ rect > < line y1 = "11" y2 = "13" x1 = "22" x2 = "22" ></ line > < line x1 = "6" y2 = "13" x2 = "6" y1 = "11" ></ line > < line y2 = "13" x1 = "10" x2 = "10" y1 = "11" ></ line > < / > } } pub const LucideBatteryMedium : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("width" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("height" , "24")] } ;