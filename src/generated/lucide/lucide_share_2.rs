use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "5" r = "3" cx = "18" ></ circle > < circle cy = "12" cx = "6" r = "3" ></ circle > < circle r = "3" cy = "19" cx = "18" ></ circle > < line x1 = "8.59" y1 = "13.51" x2 = "15.42" y2 = "17.49" ></ line > < line x1 = "15.41" x2 = "8.59" y1 = "6.51" y2 = "10.49" ></ line > < / > } } pub const LUCIDE_SHARE_2 : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("width" , "24")] } ;