use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "12" y1 = "12" x2 = "10" x1 = "6" ></ line > < line x2 = "8" y2 = "14" x1 = "8" y1 = "10" ></ line > < line x1 = "15" y1 = "13" x2 = "15.01" y2 = "13" ></ line > < line y1 = "11" x1 = "18" x2 = "18.01" y2 = "11" ></ line > < rect rx = "2" x = "2" width = "20" height = "12" y = "6" ></ rect > < / > } } pub const LUCIDE_GAMEPAD : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("height" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("width" , "24")] } ;