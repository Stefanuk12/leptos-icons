use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = ".5" cx = "7.5" fill = "currentColor" cy = "7.5" ></ circle > < circle cy = "5.5" cx = "18.5" fill = "currentColor" r = ".5" ></ circle > < circle fill = "currentColor" cx = "11.5" cy = "11.5" r = ".5" ></ circle > < circle fill = "currentColor" cy = "16.5" r = ".5" cx = "7.5" ></ circle > < circle cx = "17.5" fill = "currentColor" r = ".5" cy = "14.5" ></ circle > < path d = "M3 3v16a2 2 0 0 0 2 2h16" ></ path > < / > } } pub const LUCIDE_CHART_SCATTER : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke" , "currentColor")] } ;