use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "12" x2 = "10" y1 = "12" x1 = "6" ></ line > < line x1 = "8" y2 = "14" x2 = "8" y1 = "10" ></ line > < line y1 = "13" x1 = "15" x2 = "15.01" y2 = "13" ></ line > < line x2 = "18.01" y1 = "11" y2 = "11" x1 = "18" ></ line > < rect x = "2" width = "20" rx = "2" y = "6" height = "12" ></ rect > < / > } } pub const LUCIDE_GAMEPAD : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("height" , "24")] } ;