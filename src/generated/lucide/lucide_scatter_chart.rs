use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "7.5" r = ".5" cy = "7.5" fill = "currentColor" ></ circle > < circle cx = "18.5" fill = "currentColor" cy = "5.5" r = ".5" ></ circle > < circle fill = "currentColor" cy = "11.5" cx = "11.5" r = ".5" ></ circle > < circle cx = "7.5" cy = "16.5" r = ".5" fill = "currentColor" ></ circle > < circle cy = "14.5" cx = "17.5" fill = "currentColor" r = ".5" ></ circle > < path d = "M3 3v18h18" ></ path > < / > } } pub const LUCIDE_SCATTER_CHART : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("height" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-linejoin" , "round")] } ;