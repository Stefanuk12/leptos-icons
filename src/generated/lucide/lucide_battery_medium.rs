use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "16" y = "7" ry = "2" height = "10" x = "2" rx = "2" ></ rect > < line x1 = "22" y2 = "13" y1 = "11" x2 = "22" ></ line > < line x1 = "6" x2 = "6" y1 = "11" y2 = "13" ></ line > < line x1 = "10" x2 = "10" y2 = "13" y1 = "11" ></ line > < / > } } pub const LucideBatteryMedium : Path = Path { path : icon_path , props : & [("fill" , "none") , ("width" , "24") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-linecap" , "round")] } ;