use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "6" y1 = "12" x2 = "10" y2 = "12" ></ line > < line y1 = "10" x1 = "8" y2 = "14" x2 = "8" ></ line > < line y2 = "13" x1 = "15" x2 = "15.01" y1 = "13" ></ line > < line y2 = "11" y1 = "11" x1 = "18" x2 = "18.01" ></ line > < rect y = "6" rx = "2" height = "12" width = "20" x = "2" ></ rect > < / > } } pub const LUCIDE_GAMEPAD : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-width" , "2") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round")] } ;