use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 22V12a10 10 0 1 1 20 0v10" ></ path > < path d = "M15 6.8v1.4a3 2.8 0 1 1-6 0V6.8" ></ path > < path d = "M10 15h.01" ></ path > < path d = "M14 15h.01" ></ path > < path d = "M10 19a4 4 0 0 1-4-4v-3a6 6 0 1 1 12 0v3a4 4 0 0 1-4 4Z" ></ path > < path d = "m9 19-2 3" ></ path > < path d = "m15 19 2 3" ></ path > < / > } } pub const LUCIDE_TRAIN_FRONT_TUNNEL : Path = Path { path : icon_path , props : & [("height" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;