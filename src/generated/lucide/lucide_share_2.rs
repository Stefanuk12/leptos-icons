use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "3" cy = "5" cx = "18" ></ circle > < circle r = "3" cx = "6" cy = "12" ></ circle > < circle cy = "19" r = "3" cx = "18" ></ circle > < line x1 = "8.59" y2 = "17.49" y1 = "13.51" x2 = "15.42" ></ line > < line x1 = "15.41" y1 = "6.51" y2 = "10.49" x2 = "8.59" ></ line > < / > } } pub const LUCIDE_SHARE_2 : Path = Path { path : icon_path , props : & [("fill" , "none") , ("width" , "24") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24")] } ;