use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "6" y1 = "12" y2 = "12" x2 = "10" ></ line > < line y2 = "14" x2 = "8" x1 = "8" y1 = "10" ></ line > < line x1 = "15" y1 = "13" x2 = "15.01" y2 = "13" ></ line > < line x2 = "18.01" y1 = "11" x1 = "18" y2 = "11" ></ line > < rect width = "20" height = "12" x = "2" y = "6" rx = "2" ></ rect > < / > } } pub const LUCIDE_GAMEPAD : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("width" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("height" , "24")] } ;