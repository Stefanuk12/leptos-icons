use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "6" x2 = "10" y2 = "12" y1 = "12" ></ line > < line x2 = "8" x1 = "8" y1 = "10" y2 = "14" ></ line > < line y1 = "13" x2 = "15.01" y2 = "13" x1 = "15" ></ line > < line x1 = "18" y1 = "11" y2 = "11" x2 = "18.01" ></ line > < rect y = "6" height = "12" rx = "2" x = "2" width = "20" ></ rect > < / > } } pub const LUCIDE_GAMEPAD : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-linecap" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor")] } ;