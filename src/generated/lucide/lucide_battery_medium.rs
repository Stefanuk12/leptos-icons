use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "16" height = "10" rx = "2" ry = "2" x = "2" y = "7" ></ rect > < line y1 = "11" x2 = "22" x1 = "22" y2 = "13" ></ line > < line x1 = "6" x2 = "6" y1 = "11" y2 = "13" ></ line > < line x2 = "10" y2 = "13" x1 = "10" y1 = "11" ></ line > < / > } } pub const LucideBatteryMedium : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("width" , "24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke" , "currentColor")] } ;