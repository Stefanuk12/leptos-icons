use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16.5 6v.75m0 3v.75m0 3v.75m0 3V18m-9-5.25h5.25M7.5 15h3M3.375 5.25c-.621 0-1.125.504-1.125 1.125v3.026a2.999 2.999 0 0 1 0 5.198v3.026c0 .621.504 1.125 1.125 1.125h17.25c.621 0 1.125-.504 1.125-1.125v-3.026a2.999 2.999 0 0 1 0-5.198V6.375c0-.621-.504-1.125-1.125-1.125H3.375Z" stroke - linejoin = "round" stroke - linecap = "round" ></ path > < / > } } pub const HEROICONS_OUTLINE_TICKET : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-width" , "1.5") , ("viewBox" , "0 0 24 24") , ("aria-hidden" , "true") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("data-slot" , "icon")] } ;