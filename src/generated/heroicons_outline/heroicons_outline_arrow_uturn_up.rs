use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m9 9 6-6m0 0 6 6m-6-6v12a6 6 0 0 1-12 0v-3" stroke - linecap = "round" stroke - linejoin = "round" ></ path > < / > } } pub const HeroiconsOutlineArrowUturnUp : Path = Path { path : icon_path , props : & [("fill" , "none") , ("data-slot" , "icon") , ("stroke" , "currentColor") , ("stroke-width" , "1.5") , ("viewBox" , "0 0 24 24") , ("aria-hidden" , "true") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;